# Quasar Source Code Management

## General considerations

Quasar development team follows a light git flow approach for source control management. It is light in the sense that new code is merged directly into the `main` branch instead of an intermediary `dev` branch.

The `main` branch reflects the most recent stable state of the application code.
Anything that merges to `main` is a candidate for release.

## Pull-requests

Features or bug fixes are merged frequently to the `main` branch via pull-requests (PR) and mandatory code review.

All pull-requests should be merged with a forced merge commit, this helps for traceability as **github** will insert details about the pull-request in the commit message. The Github repo is configured to enforce this.

As much as possible, pull-requests should be rebased onto `main` before merging, this helps in creating a clean and straight history for the `main` branch.

### Review process

All pull-requests on **github** have to be approved (after review) before they can be merged.

When creating a PR, do not assign any reviewers yet. Assigning some people or all people to reviewing a PR often leads to planning issues and lack of ownership for the reviewing task.

Instead, discuss with the development team about the review (at standup) and ask who can do it. One someone is appointed reviewer, the author update the merge-request with the reviewer's name.

Usually, one reviewer is sufficient. However, if the work requires more specific knowledge, the author can ask for more than one person to review. The reviewer can also appoint someone else if agreed.

### Pull-request review material

[Official Github documentation](https://docs.github.com/en/pull-requests)
[Pull-requests review best practices](https://rewind.com/blog/best-practices-for-reviewing-pull-requests-in-github/)

## Feature & bugfix workflow

New code can be added via either a **feature** or **bugfix** branch, using the `feature/` or `bugfix/` branch name prefix respectively.

Example: create a new feature branch out of `main`

```bash
git checkout main                   # assuming not on main already
git pull                            # make sure main is in sync with remote
git checkout -b feature/feature-abc # create the new branch
```

Usually, a single developer will work on a feature branch, so it is fine to re-write the history in these branch (with `git rebase` for instance). In practice it can be used when new code has been merged to `main` in the meantime.

Example: sync new code from `main` via rebase:

```bash
git fetch --all        # fetch new code from remote origin
git rebase origin/main # rebase current branch on top of new code
```

If developers work on a feature together, they can individually branch off from the feature branch, as long as the merge is done via either a squash commit or a fast-forward commit (`--ff-only`).
By doing this, the feature branch will still be a single series of commits (no intermediate branching will be visible) and therefore the clear history visibility will be preserved.

### Visual example

```
*   0d54d68 - (3 seconds ago) Merge branch 'bugfix/bug-1' - Alice (HEAD -> main)
|\
| * 912ce23 - (15 seconds ago) Fix 3 - Alice (bugfix/bug-1)
| * d988e05 - (26 seconds ago) Fix 2 - Alice
| * e9cf5d9 - (65 seconds ago) Fix 1 - Alice
|/
*   736087f - (4 minutes ago) Merge branch 'feature/feature-1' - Bob
|\
| * b7e26f3 - (5 minutes ago) Update README with 2 - Bob (feature/feature-1)
| * 0b085c6 - (5 minutes ago) Update README with 1 - Bob
|/
* 9460178 - (8 minutes ago) Init - Alice
```

## Release process

At any point, on the `main` branch, a commit can be selected for release. For that a git tag needs to be created (see conventions).

This new git tag can be used to triggers automated CI/CD workflows that will build the release artifacts, publish and deploy them. The tag can also be used for versioning the artifacts (see versioning conventions).

If release notes and other documentation needs to be updated prior to releasing, it should be done via a **feature** branch and merged to `main`. This merge commit will be the one to be tagged to represent the new release.

Release tags can be created directly from the Github interface, by selecting the proper commit, or via command-line on a developer's machine as follows:

```bash
git checkout main    # assuming not on main already
git pull             # make sure main is in sync with remote
git tag v2.0         # create the new tag
git push origin v2.0 # push tag to remote
```

### Hotfix workflow

If a bug needs to be fixed on a version already released, a branch can be created out of the release tag, with the prefix `release/<tag>`.

Commits can then be cherry-picked to this new branch, if a fix is already available on `main`, or directly in the new branch (decide if we need a special **hotfix** branch here with mandatory PR review).

Example: create a hotfix on existing release v2.0

```bash
git checkout -b release/v2.0 v2.0 # create release branch out of tag
git push -u origin release/v2.0   # push new branch to remote
# do the hotfix work
git add .                         # make sure main is in sync with remote
git commit -m "hotfix abc"        # create the new tag
git push                          # push commit to remote release branch
```

### Visual example

```
*   9b470d3 - (8 minutes ago) Merge branch 'feature/feature-43' - Alice
|\
| * 6e4fe81 - (8 minutes ago) Work on feature-3 - Alice (feature/feature-43)
|/
| * b80093a - (3 minutes ago) hotfix 2 - Bob (release/v2.0)
| * 981ca74 - (4 minutes ago) hotfix 1 - Bob
|/
*   5522161 - (9 minutes ago) Merge branch 'feature/release-v2.0-prep' - Alice (tag: v2.0)
|\
| * b303e19 - (9 minutes ago) Prepare release v2.0 - Alice (feature/release-v2.0-prep)
|/
*   c6de55a - (14 minutes ago) Merge branch 'feature/feature-42' - Alice
|\
| * 7850c00 - (14 minutes ago) Work on feature-42 - Alice (feature/feature-42)
|/
*   0d54d68 - (19 minutes ago) Merge branch 'bugfix/bug-13' - Bob
```
