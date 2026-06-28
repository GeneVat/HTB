# HTB (HTML to BBCode)

HTB is a command-line tool that converts a subset of HTML into BBCode, suitable for use on nationstates.net

## How to Set Up

### MacOS
Download the current release folder (M-Series or Intel Series), and move the `.HTB` folder to your home directory. 
Run:
- `echo "alias HTB='~/.HTB/Release/HTB'" >> ~/.zshrc` 
- `source ~/.zshrc`

## How to Build Yourself

### MacOS
- Download the project
- Run: `cargo build --release`
- Go to: `./target/release`
- Move `HTB`, `Step1`, `Step2`, `Step3`, and `Step4` to `~/.HTB/Release` (Must create those folders)
- Run:
    - `echo "alias HTB='~/.HTB/Release/HTB'" >> ~/.zshrc` 
    - `source ~/.zshrc`
## How to Use
Use:
`HTB <FileName>`
(Takes 10-14 Seconds)