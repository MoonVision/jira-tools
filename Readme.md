[![built with nix](https://builtwithnix.org/badge.svg)](https://builtwithnix.org)

## Development environment

* Nix https://nixos.org/
* direnv https://direnv.net/
* nix-direnv https://github.com/nix-community/nix-direnv

## Authentication

JIRA API requires authentication. Credentials are provided as env variables.
In the development environment the env variables can be put into a file `.env`.
The direnv config will load variables from the `.env` file automatically.

```
JIRA_HOST=
JIRA_USER=
JIRA_PASS=
```

### `JIRA_HOST`
URL of JIRA instance. `https://teamname.atlassian.net/`

### `JIRA_USER`
Email address.

### `JIRA_PASS`
Atlassian API token https://id.atlassian.com/manage-profile/security/api-tokens.
