# Template for a rust cli

- command1: example of command without subcommands
- command2: example of command with subcommands
- generate: example of generating completion script

## How to add command
1. Create command code in the command file within the cmd directory.
1. Add the path and mod to the main.rs file.
1. Add the command into the Commands struct.
1. Add a match arm to the handle_command func to run code from the command's file.
    - if the command has subcommands add a match statement to handle them
    - typically just one func that orchestrates the commands/subcommands action with code from that file.

### Notes
- Use consistent grammer such as `verb/action noun1/subject`, `verb noun1 noun2`
- links to cli guidelines/suggestions
    - [Design Command-Line Tools People Love](https://www.youtube.com/watch?v=eMz0vni6PAw) a Gophercon video by Carolyn Van Slyck
    - [The Attributes of a Great CLI](https://packetpushers.net/the-attributes-of-a-great-cli/?doing_wp_cron=1695914431.2872240543365478515625) by Steven Iveson
