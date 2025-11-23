fn main() {
    let repo = git2::Repository::open(std::env::current_dir().expect("failed to get current path"))
        .expect("failed to read git repository");

    let types = vec!["feat", "chore", "style", "fix", "test", "refactor"];
    let selection_index = dialoguer::Select::with_theme(&dialoguer::theme::ColorfulTheme::default())
        .with_prompt("Pick a commit type")
        .items(&types)
        .default(0)
        .interact()
        .unwrap();
    let commit_type = types[selection_index];

    let commit_message: String = dialoguer::Input::with_theme(&dialoguer::theme::ColorfulTheme::default())
        .with_prompt("Commit message")
        .interact_text()
        .unwrap();

    let commit_description: String = dialoguer::Input::with_theme(&dialoguer::theme::ColorfulTheme::default())
        .with_prompt("Commit description")
        .allow_empty(true)
        .interact_text()
        .unwrap();

    let stage_all_changes = dialoguer::Confirm::with_theme(&dialoguer::theme::ColorfulTheme::default())
        .with_prompt("Stage all changes?")
        .default(true)
        .interact()
        .unwrap();

    commit(&repo, commit_type, &commit_message, &commit_description, stage_all_changes)
        .expect("failed to commit");
}

fn commit(repo: &git2::Repository, commit_type: &str, commit_message: &str, commit_description: &str, stage_all_changes: bool) -> Result<(), git2::Error> {
    let mut index = repo.index()?;

    if stage_all_changes {
        index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
    }

    index.write()?;
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;
    let sig = repo.signature()?;
    let parent_commit = match repo.head() {
        Ok(reference) => reference.peel_to_commit().ok(),
        Err(_) => None,
    };

    let message = format!("{}: {} \n\n{}", commit_type, commit_message, commit_description);

    let commit_id = if let Some(parent) = parent_commit {
        repo.commit(Some("HEAD"), &sig, &sig, &message, &tree, &[&parent])?
    } else {
        repo.commit(Some("HEAD"), &sig, &sig, &message, &tree, &[])?
    };

    println!("Created commit {:?}", commit_id);
    Ok(())
}