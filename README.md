# Dailog script and voiceline builder

For the purpose of generating Dailog videos, the following pipeline was designed:
```
    -----------------          ---------------            ---------
    | script.dailog | -------> | script.json | ---------> | *.mp3 |
    -----------------  Rust    ---------------  Python    ---------
```
where, the `.dailog` file denotes an easy-to-use representation of the script that gets parsed and transformed into a `.json` file. This first transformation is performed using `Rust` in the [script_builder](./script_builder/) project. Thereafter, the `.json` file is used to generate a collection of `.mp3` files adhering to the script, voice, and order. For example, the used voices of the avatars are denoted with `B`, `J`, `D`, that represent the virtual avatars of Bill, Jill, and Dee, respectively. This is done in the [dailog_aws_polly](./dailog_aws_polly/) project.   
