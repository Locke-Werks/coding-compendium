---
id: terraform
title: Terraform (HCL)
type: language
verified: 2026-08-02
volatility: quarterly

name: Terraform
aka: [hcl, hashicorp configuration language, opentofu, tf]
family: config
likelihood: possible
extensions: ['.tf', '.tfvars', '.hcl']

danger: >
  `terraform destroy` deletes every piece of cloud infrastructure the project
  manages, including databases and their contents. `terraform apply` can do the
  same thing quietly: changing a name or another fixed attribute produces a plan
  that replaces a resource rather than editing it. Always run `terraform plan`
  first and read it for the phrases `must be replaced` and `to destroy`. Those
  words mean data loss, not an update.

tells:
  - pattern: '^(resource|data|provider|variable|output|module)\s+"'
    kind: regex
    weight: 10
    note: >
      A block is a bare keyword followed by one or two quoted labels and a brace:
      `resource "aws_s3_bucket" "assets" {`. JSON would need quotes and a colon,
      YAML would need a colon and indentation, and neither allows two labels.
  - pattern: '=\s*(var|local|module|data)\.'
    kind: regex
    weight: 8
    note: >
      Values reference other blocks by bare path: `region = var.region`. YAML and
      JSON cannot refer to another key at all, which is the whole reason this
      format exists.
  - pattern: '^\s*\w+\s*=\s*'
    kind: regex
    weight: 5
    note: >
      Attributes are `key = value` with no quotes on the key and no comma at the
      end. JSON demands both. TOML looks similar, but TOML groups with `[table]`
      headers instead of braces.
  - pattern: '\$\{'
    kind: regex
    weight: 4
    note: >
      String interpolation, `"${var.name}-prod"`. Bash uses `${VAR}` as well, so
      pair this with a `resource` block before calling it.

rules_out:
  - pattern: '^\s*"[^"]+":'
    kind: regex
    because: >
      JSON. A quoted key with a colon is not HCL syntax.
  - pattern: '^---'
    kind: line_start
    because: >
      YAML. A document separator, so the file is probably Kubernetes or a
      workflow.
  - pattern: '^\['
    kind: line_start
    because: >
      TOML or INI. Those group settings under a bracketed heading.

project_fingerprint:
  manifests:
    - file: '*.tf'
      decisive: true
      note: >
        Any file with this extension makes it Terraform. All `.tf` files in a
        folder are read together as one configuration, so splitting them is
        cosmetic.
    - file: 'main.tf'
      note: >
        The conventional entry point, usually beside `variables.tf` and
        `outputs.tf`.
    - file: 'terraform.tfvars'
      note: >
        The actual values for this deployment. Frequently holds secrets and
        frequently ends up in `.gitignore` for that reason.
    - file: 'terraform.tfstate'
      note: >
        The record of what has been created. Never edit it by hand and never
        commit it: it contains secrets in plain text and losing it means
        Terraform forgets what it owns.
  lockfiles: ['.terraform.lock.hcl']
  build_dirs: ['.terraform/']
  entry_points: ['main.tf']

shape:
  blocks: braces
  statement_end: newline
  comment_line: '#'
  comment_block: '/* */'
  string_quotes: >
    Double quotes only. Single quotes are a syntax error, which catches everyone
    arriving from another language.
  naming: snake_case for names, and resource types are provider-prefixed like aws_instance
  import_keyword: module

confusable_with:
  - language: json
    settle_it: >
      Terraform can be written as JSON, so check the keys. Bare unquoted keys with
      `=` between key and value are HCL. Quoted keys with `:` and trailing commas
      are JSON.
    tiebreak: { pattern: '^\s*\w+\s*=\s*', kind: regex, favors: terraform }
  - language: yaml
    settle_it: >
      Both describe infrastructure and both are read more than written. YAML uses
      indentation and colons and has no braces. HCL uses braces and `=`.
    tiebreak: { pattern: '^\s+\w+:\s', kind: regex, favors: yaml }

errors_look_like:
  sample: |
    Error: Unsupported argument

      on main.tf line 14, in resource "aws_s3_bucket" "assets":
      14:   acl = "private"

    An argument named "acl" is not expected here.
  recognize_by: >
    The word `Error:` on its own line, a blank line, then an indented location in
    the form `on <file> line <n>, in <block>`, then the offending line reprinted
    with its number. The explanation comes last, which is the opposite of most
    compilers.
  patterns:
    - '^Error: '
    - 'on .*\.tf line \d+, in '
    - 'Terraform (has|will) '

meet_it_when: >
  You inherit a repo that provisions its own hosting, or an agent offers to set up
  cloud infrastructure and writes `main.tf`. You are unlikely to write it from
  scratch and likely to have to read a plan before approving it.

what_agents_get_wrong: >
  Agents invent arguments that a provider does not have, which `terraform plan`
  catches for you, and they hardcode credentials into a `.tf` file, which nothing
  catches until it is committed. The expensive mistake is quieter: renaming a
  resource or changing an attribute that cannot be updated in place produces a
  plan that destroys and recreates it. For a bucket that means downtime, for a
  database that means the data is gone. Read the plan output for `must be
  replaced` and for the destroy count on the summary line before typing yes.

see_also:
  - json
  - yaml
  - j2-the-config-formats-nobody-explains

keywords: [hcl, infrastructure as code, opentofu, provider, tfstate, plan, apply]
---

Terraform describes cloud infrastructure as text files, then makes reality match
them. The language those files are written in is HCL (HashiCorp Configuration
Language), and you will see the same syntax in other HashiCorp tools.

The shape is distinctive: a keyword, one or two quoted labels, then a brace. No
commas, no quoted keys, and `=` between a name and its value.

```hcl
resource "aws_s3_bucket" "assets" {
  bucket = "nyx-assets-prod"
  tags = {
    Environment = var.environment
  }
}
```

`var.environment` reaches into a `variable` block declared elsewhere in the same
folder. That ability to point at another value is what separates this from JSON
(JavaScript Object Notation) and the other config formats, which can only hold
what you typed into them. See [JSON](#json) if that one is new.

The workflow is always the same three commands: `terraform init` downloads the
providers, `terraform plan` prints what it would change, `terraform apply` does it.
The plan is the review step, and it is the only one that costs nothing.
