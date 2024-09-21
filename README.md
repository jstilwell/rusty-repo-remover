# Rusty Repo Remover

**PERMANENTLY** delete a pre-configured list of repositories from your Github Account. Be sure that you actually want to permanently remove these repositories before running this program with the --delete flag!

# Getting Started

- Rename ./config/config.toml.example to ./config/config.toml
- Generate a Github personal access token
  - Log in to Github
  - Click your avatar at the top right
  - Click "Settings"
  - Click "Developer Settings" at the bottom
  - Click "Personal access tokens"
  - Click "Fine-grained tokens"
  - Click "Generate new token"
  - Title it "rusty-repo-remover"
  - Set an expiration date — 7 days is the most secure.
    - You should also just revoke this token after you run the deletions.
  - Select the appropriate repository access level
  - Under repository permissions, set "Administration" to Read and Write
  - Click "Generate Token"
- Copy and paste the token into ./config/config.toml as the token value
- Set your username as the user value
- Add some repositories (see list command below) to delete to ./config/config.toml

# Commands

Update Cargo:

```bash
cargo update
```

List all of your repositories:

```bash
cargo run -- list
```

Perform a dry-run of the repo deletions:

```bash
cargo run
```

**PERMANENTLY DELETE** the listed repos:

```bash
cargo run -- delete
```
