use base64::Engine;

#[derive(Debug, PartialEq)]
enum VersionToUpdate {
    Major,
    Minor,
    Patch,
    None,
}

#[derive(Debug, Clone, Copy)]
enum RepoType {
    Typescript,
    Java,
}

const REPO_FILES: &[(RepoType, &str)] = &[
    (RepoType::Typescript, "package.json"),
    (RepoType::Java, "pom.xml"),
];

// Function to find update based on commit messages
fn find_update(commit_messages: Vec<&str>) -> VersionToUpdate {
    let mut feature = 0;
    let mut bugfix = 0;

    for commit_message in commit_messages {
        if commit_message.contains("!") {
            return VersionToUpdate::Major;
        }
        if commit_message.starts_with("feat") {
            feature += 1;
        }
        if commit_message.starts_with("fix") {
            bugfix += 1;
        }
    }
    if feature > 0 {
        VersionToUpdate::Minor
    } else if bugfix > 0 {
        VersionToUpdate::Patch
    } else {
        VersionToUpdate::None
    }
}

// Function to get version update string
fn get_version_update_string(version_to_update: &VersionToUpdate) -> &'static str {
    match version_to_update {
        VersionToUpdate::Major => "major",
        VersionToUpdate::Minor => "minor",
        VersionToUpdate::Patch => "patch",
        _ => "",
    }
}

// Function to run shell commands
fn run_shell_command(command: &str) -> Result<String, Box<dyn std::error::Error>> {
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()?;
    let result = String::from_utf8(output.stdout)?;
    Ok(result)
}

// Function to update Java version
fn update_java_version(
    version_to_update: &VersionToUpdate,
) -> Result<String, Box<dyn std::error::Error>> {
    let current_version = run_shell_command(
        "cd tmp && mvn help:evaluate -Dexpression=project.version -q -DforceStdout",
    )?
    .replace("-SNAPSHOT", "");
    let version_parts: Vec<&str> = current_version.split('.').collect();
    let mut version: Vec<String> = vec![];

    match version_to_update {
        VersionToUpdate::Major => {
            version.push((version_parts[0].parse::<u32>()? + 1).to_string());
            version.push("0".to_string());
            version.push("0".to_string());
        }
        VersionToUpdate::Minor => {
            version.push((version_parts[0].parse::<u32>()?).to_string());
            version.push((version_parts[1].parse::<u32>()? + 1).to_string());
            version.push("0".to_string());
        }
        VersionToUpdate::Patch => {
            version.push((version_parts[0].parse::<u32>()?).to_string());
            version.push((version_parts[1].parse::<u32>()?).to_string());
            version.push((version_parts[2].parse::<u32>()? + 1).to_string());
        }
        _ => {}
    }
    let new_version = version.join(".");
    run_shell_command(&format!(
        "cd tmp && echo \"{}-SNAPSHOT\" | mvn release:update-versions",
        new_version
    ))?;
    Ok(new_version)
}

// Function to update and get the new version
async fn update_and_get_new_version(
    repo_type: RepoType,
    version_to_update: &VersionToUpdate,
) -> String {
    match repo_type {
        RepoType::Typescript => {
            let cmd = format!(
                "cd tmp && npm version {} --no-git-tag-version",
                get_version_update_string(version_to_update)
            );
            run_shell_command(&cmd).unwrap_or_else(|_| "error".to_string())
        }
        RepoType::Java => {
            update_java_version(version_to_update).unwrap_or_else(|_| "error".to_string())
        }
    }
}

// Function to get the new version
async fn get_new_version(
    octokit: &octocrab::Octocrab,
    repo_type: RepoType,
    owner: &str,
    repo: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let commits = octokit.repos(owner, repo).list_commits().send().await?;
    let mut commit_messages: Vec<&str> = Vec::new();
    let mut merge_commit = 0;

    for commit in commits.items.iter() {
        let message = &commit.commit.message;
        if message.starts_with("Merge pull request") {
            merge_commit += 1;
            if merge_commit >= 2 {
                break;
            }
        } else {
            commit_messages.push(message.as_str());
        }
    }
    let version_to_update = find_update(commit_messages);
    if version_to_update == VersionToUpdate::None {
        return Ok("none".to_string());
    }
    Ok(update_and_get_new_version(repo_type, &version_to_update).await)
}

async fn update_version(
    octokit: &octocrab::Octocrab,
    repo_type: RepoType,
    owner: &str,
    repo: &str,
    file_content: octocrab::models::repos::Content,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create file locally
    let start_decode = std::time::Instant::now();
    let file_content_decoded = match base64::engine::general_purpose::STANDARD
        .decode(file_content.content.unwrap().replace("\n", ""))
    {
        Ok(decoded_content) => String::from_utf8(decoded_content)?,
        Err(e) => return Err(Box::new(e)),
    };
    println!("Time elapsed to decode: {:?}", start_decode.elapsed());
    let (file_path, filename) = match repo_type {
        RepoType::Java => ("tmp/pom.xml", "pom.xml"),
        RepoType::Typescript => ("tmp/package.json", "package.json"),
    };
    let start_write_file = std::time::Instant::now();
    tokio::fs::write(file_path, file_content_decoded).await?;
    println!(
        "Time elapsed to write file: {:?}",
        start_write_file.elapsed()
    );
    let start_get_version = std::time::Instant::now();
    let version = get_new_version(octokit, repo_type, owner, repo).await?;
    println!(
        "Time elapsed to get version file: {:?}",
        start_get_version.elapsed()
    );
    if version == "none" || version == "error" {
        println!("No need to update or error while getting the version");
        return Ok(());
    }
    let updated_content = tokio::fs::read_to_string(file_path).await?;
    let start_push_commit = std::time::Instant::now();
    let requete = octokit
        .repos(owner, repo)
        .update_file(
            filename,
            "chore: Update version",
            updated_content,
            file_content.sha,
        )
        .send()
        .await;
    println!("{:#?}", requete);
    println!(
        "Time elapsed to push commit file: {:?}",
        start_push_commit.elapsed()
    );
    // Create an annotated tag
    Ok(())
}

pub async fn webhook_handler(
    axum::extract::State(state): axum::extract::State<
        std::sync::Arc<crate::inbound::http::AppState>,
    >,
    payload: axum::Json<crate::github::payload::Payload>,
) -> impl axum::response::IntoResponse {
    let repository_full_name: &str = payload.repository().full_name();
    let pusher_name: &str = payload.pusher().name();
    let payload_ref: &str = payload.payload_ref();
    if payload.head_commit().is_none() {
        println!(
            "{} - The commit is empty, it's most likely a commit to delete the branch {}",
            repository_full_name, payload_ref
        );
        return "";
    }
    if let Some(head_commit) = payload.head_commit() {
        let commiter_name: &str = head_commit.committer().name();
        if commiter_name != "GitHub Enterprise" {
            println!(
                "{} - Receive a push event by {}",
                repository_full_name, pusher_name
            );
            println!(
                "{} - The branch where the commit was push is {}",
                repository_full_name, payload_ref
            );
            println!(
                "{} - The commit message is {}\n\tand the commiter is {}",
                repository_full_name,
                head_commit.message(),
                commiter_name
            );
        } else if commiter_name == "GitHub Enterprise" && payload_ref == "refs/heads/main" {
            println!(
                "{} - The github app will update the version (if necessary)",
                repository_full_name
            );
            println!(
                "{} - It will create a tag (if necessary)",
                repository_full_name
            );
            println!(
                "{} - And a release with the new tag (if necessary) (not yet discussed)",
                repository_full_name
            );
            let mut promises: Vec<Result<octocrab::models::repos::ContentItems, octocrab::Error>> =
                vec![];
            for repo_type in REPO_FILES {
                let start_request_get_content = std::time::Instant::now();
                promises.push(
                    state
                        .github_app
                        .repos(
                            payload.repository().owner().name().unwrap(),
                            payload.repository().name(),
                        )
                        .get_content()
                        .path(repo_type.1)
                        .send()
                        .await,
                );
                println!(
                    "Time elapsed for request get_content {:?}",
                    start_request_get_content.elapsed()
                );
            }
            let successful_promises: Vec<octocrab::models::repos::ContentItems> = promises
                .into_iter()
                .filter_map(|promise| promise.ok())
                .collect();
            if successful_promises.len() > 1 {
                println!("Not yet supported !");
            } else if successful_promises.is_empty() {
                eprintln!("No file found or repo type not yet supported");
            } else {
                let start_request_get_content = std::time::Instant::now();
                let content = successful_promises
                    .first()
                    .unwrap()
                    .to_owned()
                    .take_items()
                    .first()
                    .unwrap()
                    .to_owned();
                match content.name.as_str() {
                    "pom.xml" => {
                        println!("Java repo");
                        let _ = update_version(
                            &state.github_app,
                            RepoType::Java,
                            payload.repository().owner().name().unwrap(),
                            payload.repository().name(),
                            content,
                        )
                            .await;
                        let _ = run_shell_command("cd tmp ; rm pom.xml");
                    }
                    "package.json" => {
                        println!("Javascript repo");
                        let start_update_version = std::time::Instant::now();
                        let _ = update_version(&state.github_app, RepoType::Typescript, payload.repository().owner().name().unwrap(), payload.repository().name(), content).await;
                        println!(
                            "Time elapsed for update {:?}",
                            start_update_version.elapsed()
                        );
                        let _ = run_shell_command("cd tmp ; rm package.json");
                    },
                    _ => return "This repo type is not implemented yet, please open a Issue on Github",
                }
                println!(
                    "Time elapsed for create file and update {:?}",
                    start_request_get_content.elapsed()
                );
            }
        }
    }
    "It's working"
}
