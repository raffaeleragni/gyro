# gyro

CLI for workflow patterns used in software management (eg JIRA, ...).

## How to setup the environment

Set the environment variables: `JIRA_API_HOST`, `JIRA_API_USER`, `JIRA_API_TOKEN`. For the cloud JIRA API v3 the host is "https://<youraccount>.atlassian.net".

The currently selected ticket is expected to be in GYRO_KEY.

## Commands

### `gyro`

Shows info about the selected key.

```
$ gyro
!no key selected!
$ export GYRO_KEY="PROJECT-1"
$ gyro
PROJECT-1: Ticket title
```
