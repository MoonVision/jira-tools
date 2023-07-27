[![built with nix](https://builtwithnix.org/badge.svg)](https://builtwithnix.org)

## Development environment

* Nix https://nixos.org/
* direnv https://direnv.net/
* nix-direnv https://github.com/nix-community/nix-direnv

## Authentication

JIRA API requires authentication.


### Config File

`jira-tools` can be configured with a config file in
`~/.config/jira-tools/jira-tools.toml` or in your current working directory (`./jira-tools.toml`).

```toml
host = "..."
username = "..."
password = "..."
```

### Env Variables

In the development environment the env variables can be put into a file `.env`.
The direnv config will load variables from the `.env` file automatically.

```
JIRA_HOST=
JIRA_USERNAME=
JIRA_PASSWORD=
```

### `JIRA_HOST`
URL of JIRA instance. `https://teamname.atlassian.net/`

### `JIRA_USERNAME`
Email address.

### `JIRA_PASSWORD`
Atlassian API token https://id.atlassian.com/manage-profile/security/api-tokens.
