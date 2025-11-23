fn main() {
    println!("gcommit: \n");

    let tasks = vec![
        "✅ get path to repository",
        "✅ check if it exists and is a git repository by checking .git directory's existence",
        "✅ get commit type",
        "✅ get commit message",
        "✅ get commit description",
        "🕓 make a commit using git2",
        "🕓 ask wether to push it and show a selector between remotes",
    ];

    for (i, task) in tasks.iter().enumerate() {
        println!("  {}. {}", i + 1, task);
    }

    let path = std::env::current_dir().expect("failed to get current path");
    let is_git_repo = path.join(".git").is_dir();

    if !is_git_repo {
        eprintln!("Error: not a git repository");
        std::process::exit(1);
    }

    let commit_type = match get_input("please pick a commit type: \n1. feat\n2. chore\n3. style\n4. fix\n5. test\n6. refactor").as_str() {
        "1" => "feat",
        "2" => "chore",
        "3" => "style",
        "4" => "fix",
        "5" => "test",
        "6" => "refactor",
        _ => {
            println!("Error: wrong input");
            return;
        }
    };

    let commit_message = get_input("please type your commit message");
    let commit_description = get_input("please type your commit description");

    let stage_all_changes = match get_input("would you like to stage all changes: \n1. yes\n2. no").as_str() {
        "1" => true,
        "2" => false,
        _ => {
            println!("Error: wrong input");
            return;
        }
    };

    let final_message = format!("{}: {} \n\n{}", commit_type, commit_message, commit_description);

    let repo = git2::Repository::open(path).expect("failed to read git repository");

    commit(&repo, &final_message, stage_all_changes).expect("failed to commit");

    println!("\nDone!");
}

fn get_input(question: &str) -> String {
    println!("{}", question);
    let mut input = "".to_string();
    std::io::stdin().read_line(&mut input).expect("failed to read input");
    return input.trim().to_string();
}

fn commit(repo: &git2::Repository, message: &str, stage_all_changes: bool) -> Result<(), git2::Error> {
    let mut index = repo.index()?;

    // Optionally stage all
    if stage_all_changes {
        index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
    }

    index.write()?; // write index to disk

    // Write tree from index
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;

    let sig = repo.signature()?; // uses git user.name & user.email config

    // Get HEAD commit if exists
    let parent_commit = match repo.head() {
        Ok(reference) => reference.peel_to_commit().ok(),
        Err(_) => None,
    };

    let commit_id = if let Some(parent) = parent_commit {
        repo.commit(Some("HEAD"), &sig, &sig, message, &tree, &[&parent])?
    } else {
        repo.commit(Some("HEAD"), &sig, &sig, message, &tree, &[])?
    };

    println!("Created commit {:?}", commit_id);
    Ok(())
}