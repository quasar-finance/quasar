# Quasar Development Process

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

```
