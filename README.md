# gyro

CLI for workflow patterns used in software management (eg JIRA, ...).

## How to setup the environment

Set the environment variables: `JIRA_API_HOST`, `JIRA_API_USER`, `JIRA_API_TOKEN`. For the cloud JIRA API v3 the host is "https://<youraccount>.atlassian.net".

The currently selected ticket is expected to be in GYRO_KEY.

## Commands

### `gyro (s)how`

Shows info about the selected key.

```
$ gyro s
!no key selected!
$ export GYRO_KEY="PROJECT-1"
$ gyro s
PROJECT-1: Ticket title
```

### `gyro (k)ey`

Search for a key to select. Currently only works if the result is unique.

```
$ gyro k title
PROJECT-1
$ export GYRO_KEY="`gyro k title`"
$ gyro s
PROJECT-1: Ticket title
```
