name: Bug report
about: Report a problem with a toolkit command
title: "[bug] "
labels: bug
body:
  - type: textarea
    id: command
    attributes:
      label: Command
      description: Which command and input produced the problem?
    validations:
      required: true