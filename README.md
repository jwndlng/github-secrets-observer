# GitHub Secrets Observer

## Overview
GitHub Secrets Observer is an application designed to enhance security practices around the use of GitHub Secrets by monitoring the rotation of secrets across GitHub repositories.

## Motivation
Many organizations rely on GitHub for code management and use GitHub Secrets for secret management within GitHub Workflows. However, these secrets often are not rotated regularly, posing a security risk. This tool aims to automate secret rotation notifications, ensuring better compliance with security best practices.

## Getting started

### Download the latest release

Browse the [releases](https://github.com/jwndlng/github-secrets-observer/releases) and download the latest version. Follow the configuration guidances in the next section and run it!

```bash
Usage: github-secrets-observer [OPTIONS]

Options:
  -o, --organization <ORGANIZATION>    
  -l, --log-level <LOG_LEVEL>          [default: INFO]
  -n, --notifier-type <NOTIFIER_TYPE>  [possible values: slack, github, log]
  -h, --help                           Print help
```

### Example Output

```bash
2024-06-....  INFO github_secrets_observer: Loading settings.
2024-06-....  INFO github_secrets_observer: Observer initialized successfully.
2024-06-....  INFO github_secrets_observer::notifier: 🤷 [state=Ignored, name=SECRET1, repository=my_org/my_repo, days_age=101, days_left=0, days_overdue=0] Secret is ignored.
2024-06-.... ERROR github_secrets_observer::notifier: ❌ [state=Expired, name=SECRET2, repository=my_org/my_repo, days_age=170, days_left=0, days_overdue=80] Secret is expired.
2024-06-....  INFO github_secrets_observer: Observer finished successfully.
```

### Configuration

The following table lists all of the options to configure the application

| Name                         | Section   | Required | Default    | Note                                                                                         |
| ---------------------------- | --------- | -------- | ---------- | -------------------------------------------------------------------------------------------- |
| organization                 | github    | Yes      | -          | The GitHub organization that will be audited.                                                |
| token                        | github    | Yes      | -          | The GitHub access token. Don't use the configuration file and use environments instead.      |
| default_rotation_days        | observer  | No       | 90         | Default rotation in days for tokens that don't use the suffix. E.g. use `_R10` for 10 days.  |
| expiration_notice_days       | observer  | No       | 14         | Default notice days, when a secret is considered to expire soon.                             |         
| ignore_pattern               | observer  | No       | -          | Regex pattern that allows to ignore secrets from the scan that match the regex.              |
| ignore_secrets               | observer  | No       | -          | List of secrets that will be ignored.                                                        |
| notifier_type                | notifier  | No       | log        | Method that is used to notify about secrets.                                                 |
| slack_webhook                | notifier  | No       | -          | Use Slack Webhook for notifications. Enable by setting an URL.                               |

Each option can be either configured via the `config.toml` file or environment variables. Both can be used for different options.

ℹ️ **Info:** CLI arguments will override the settings.

#### Using the configuration file
The configuration file uses the TOML format. The current configuration does not use a nested pattern. So each section contains the options listed above. 

#### Using environment variables
The environment variables must use the prefix `GHSO` and follow the pattern `Prefix_Section_Name`. For the organization the environment variable would be `GHSO_GITHUB_ORGANIZATION`.
