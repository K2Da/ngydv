# ngydv
ngydv creates aws cli session keys with mfa-token and stores, exports it.

## Installation
### Using cargo

```bash
cargo install ngydv
```

## Usage
You need aws cli configuration file ~/.aws/config and ~/.aws/credentials.

And at more file at ~/.aws/ngydv to store your mfa device arn, if needed.

```~/.aws/ngydv
# ~/.aws/ngydv
[profile_a]
  mfa_device = arn:aws:iam::nnnnnnnnnnnn:mfa/user_name
```

## Sub commands
### create session / assume role
Creates session or assume role based on profile type and store it in ~/.aws/ngydv_credentials.yaml.

And prints commands to export the keys and token, so use like this.

```bash
# ngydv in {profile} {mfa_token}
source <(ngydv in profile_a 000000)
# or
. <(ngydv in profile_a 000000)
```

#### create session
Creates session for profile like this.

``` .aws/credentials
# .aws/credentials
[profile_a]
aws_access_key_id = XXXXXXXXXXXXXXXXXXXX
aws_secret_access_key = xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
```

``` .aws/config
# .aws/config
[profile_a]
  output = yaml
  region = ap-northeast-1
```

```~/.aws/ngydv
# ~/aws/ngydv
[profile_a]
  mfa_device = arn:aws:iam::nnnnnnnnnnnn:mfa/user_name
```

#### assume role
Assume role for profile like this.

``` .aws/config
# .aws/config
[profile_b]
  region = ap-northeast-1
  role_arn = arn:aws:iam::nnnnnnnnnnn:role/rolename
  mfa_serial = arn:aws:iam::mmmmmmmmmmmm:mfa/user-name
  source_profile = base_profile
```

### export
This sub command prints sh commands to export stored session tokens.

The output is same as 'in' sub command, but no need to input mfa token again. This sub command is to switch between multi user profile or multi aws account.

```
source <(ngydv export profile)
# or
. <(ngydv export profile)
```

### clear session
Clears stored sessions in ~/.aws/ngydv_credentials.yaml.

```
ngydv clear session
```

### clear env
Clears environment variables related AWS CLI command.

```
. <(ngydv clear env)
# or
source <(ngydv clear env)
```

### profile
list available profiles.

```
$ ngydv profile
 id | profile   | region         | type                       | credential
----+-----------+----------------+----------------------------+---------------------------------------
 1  | default   | ap-northeast-2 | Access key                 | -
 2  | hpm       | ap-northeast-1 | Access key with mfa device | expired at 2020-05-06 00:02:01 +09:00
 3  | hpmadm    | ap-northeast-1 | Assume role from hpm       | -
```

### env
list environment variables related AWS CLI.

```
$ ngydv env
 name                        | desc                                                                                     | default            | value
-----------------------------+------------------------------------------------------------------------------------------+--------------------+------------------------------------------
 AWS_ACCESS_KEY_ID           | AWS access key associated with an IAM user or role                                       |                    | XXXXXXXXXXXXXXXXXXXX
 AWS_CA_BUNDLE               | The path to a certificate bundle to use for HTTPS certificate validation                 |                    |
 AWS_CONFIG_FILE             | The location of the file that the AWS CLI uses to store configuration profiles           | ~/.aws/config      |
 AWS_DEFAULT_OUTPUT          | The output format to use                                                                 | json               |
 AWS_DEFAULT_REGION          | The AWS Region to send the request to                                                    |                    |
 AWS_PAGER                   | The pager program used for output                                                        |                    |
 AWS_PROFILE                 | The name of the CLI profile with the credentials and options to use                      | default            |
 AWS_ROLE_SESSION_NAME       | A name to associate with the role session                                                |                    |
 AWS_SECRET_ACCESS_KEY       | The secret key associated with the access key                                            |                    | xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
 AWS_SESSION_TOKEN           | The session token value that is required if you are using temporary security credentials |                    |
 AWS_SHARED_CREDENTIALS_FILE | The location of the file that the AWS CLI uses to store access keys                      | ~/.aws/credentials |
```
