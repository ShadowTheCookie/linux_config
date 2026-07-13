#Based on the GZNH Theme ohmyzsh theme found at https://github.com/ohmyzsh/ohmyzsh/blob/master/themes/gnzh.zsh-theme


setopt prompt_subst

() {

local PR_USER PR_USER_OP PR_PROMPT PR_HOST
local LINE_COLOR

# Bordeaux / Ember Coral palette
local DARK_AMARANTH='%F{88}'   # #720026
local AMARANTH='%F{168}'       # #ce4257
local DEEP_CORAL='%F{203}'     # deeper coral / pink-coral

# Make prompt lines match username color
LINE_COLOR="${DEEP_CORAL}"

# Check UID
if [[ $UID -ne 0 ]]; then
  # Normal user
  PR_USER="${DEEP_CORAL}%n%f"
  PR_USER_OP="${DEEP_CORAL}%#%f"
  PR_PROMPT="${DEEP_CORAL}➤ %f"
else
  # Root user
  PR_USER="${AMARANTH}%n%f"
  PR_USER_OP="${AMARANTH}%#%f"
  PR_PROMPT="${AMARANTH}➤ %f"
fi

# SSH detection
if [[ -n "$SSH_CLIENT" || -n "$SSH2_CLIENT" ]]; then
  PR_HOST="${AMARANTH}%M%f"
else
  PR_HOST="${AMARANTH}%m%f"
fi

# Return code
local return_code="%(?..${AMARANTH}%? ↵%f)"

# Prompt components
local user_host="${PR_USER}${AMARANTH}@${PR_HOST}"
local current_dir="%B${DEEP_CORAL}%~%f%b"
local git_branch='$(git_prompt_info)'
local venv_prompt='$(virtualenv_prompt_info)'

PROMPT="${LINE_COLOR}╭─%f${venv_prompt}${user_host} ${current_dir} \$(ruby_prompt_info) ${git_branch}
${LINE_COLOR}╰─%f${PR_PROMPT}"

RPROMPT="${return_code}"

# Git colors
ZSH_THEME_GIT_PROMPT_PREFIX="${AMARANTH}‹"
ZSH_THEME_GIT_PROMPT_SUFFIX="›%f"

# Ruby colors
ZSH_THEME_RUBY_PROMPT_PREFIX="${AMARANTH}‹"
ZSH_THEME_RUBY_PROMPT_SUFFIX="›%f"

# Virtualenv colors
ZSH_THEME_VIRTUALENV_PREFIX="${DARK_AMARANTH}("
ZSH_THEME_VIRTUALENV_SUFFIX=")%f "

}
