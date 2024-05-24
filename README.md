# GitHub Secrets Observer

## Overview
GitHub Secrets Observer is an application designed to enhance security practices around the use of GitHub Secrets by monitoring the rotation of secrets across GitHub repositories.

## Motivation
Many organizations rely on GitHub for code management and use GitHub Secrets for secret management within GitHub Workflows. However, these secrets often are not rotated regularly, posing a security risk. This tool aims to automate secret rotation notifications, ensuring better compliance with security best practices.

## Getting started

### Download the latest release (TBD)

Browse the [releases](https://github.com/jwndlng/github-secrets-observer/releases) and download the latest version. Follow the configuration guidances in the next section and run it!

### Configuration

The following table lists all of the options to configure the application

| Name                         | Section              | Required     | Note                                                                                         |
| ---------------------------- | -------------------- | ------------ | -------------------------------------------------------------------------------------------- |
| organization                 | github               | Yes          | The GitHub organization that will be audited.                                                |
| token                        | github               | Yes          | The GitHub access token. Don't use the configuration file and use environments instead.      |
| default_rotation             | observer             | No           | Default rotation in days for tokens that don't use the suffix. E.g. use `_R10` for 10 days.  |
| ignore_pattern               | observer             | No           | Regex pattern that allows to ignore secrets from the scan that match the regex.              |
| ignore_secrets               | observer             | No           | List of secrets that will be ignored.                                                        |

Each option can be either configured via the `config.toml` file or environment variables. Both can be used for different options.

#### Using the configuration file
The configuration file uses the TOML format. The current configuration does not use a nested pattern. So each section contains the options listed above. 

#### Using environment variables

The environment variables must use the prefix `GHSO` and follow the pattern `Prefix_Section_Name`. For the organization the environment variable would be `GHSO_GITHUB_ORGANIZATION`.
