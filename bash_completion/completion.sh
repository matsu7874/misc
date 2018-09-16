# bash completion for command.py
# [Bashタブ補完自作入門 - Cybozu Inside Out | サイボウズエンジニアのブログ](https://blog.cybozu.io/entry/2016/09/26/080000)
# ~/.bash_completion 下記の内容を書くと補完が効くようになる。

_except() {
  # 配列を集合と見たときの差集合`a - b`を返す
  local a b intersection difference
  a=$1
  b=$2

  intersection=($(for item in ${a[@]} ${b[@]}; do echo "$item"; done | sort | uniq -d))
  difference=($(for item in ${a[@]} ${intersection[@]}; do echo "$item"; done | sort | uniq -u))
  echo "${difference[@]}"
}


_command() {
  local cur prev cword opts numbers animals unused_opts
  _get_comp_words_by_ref -n : cur prev cword
  opts="--number --animal"
  unused_opts="$(_except "$opts" "${COMP_WORDS[*]}")"
  numbers="zero one two three four five six seven eight nine"
  animals="cat dog deer"


  COMPREPLY=( $(compgen -W "${unused_opts[@]}" -- "${cur}") )
  if [ "${prev}" = "--number" ]; then
    COMPREPLY=( $(compgen -W "${numbers}" -- "${cur}") )
  elif [ "${prev}" = "--animal" ]; then
    COMPREPLY=( $(compgen -W "${animals}" -- "${cur}") )
  fi
}

complete -F _command command.py
