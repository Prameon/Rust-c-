# Error_Глава 1
P.s. Мы писали только в консоли, так что ошибки только связаные с консолью.(я использовал консоль(терминал) powershell(Windows 10 в программе VS Code)).

 error: could not find `Cargo.toml` in `D:\Coding\Rust` or any parent directory
>Решиение: 
>
>Не зашёл в папку с проектом
```
$ cd name
```
---
error: destination `D:\Coding\Rust\name` already exists

Use `cargo init` to initialize the directory
>Решиение: 
>
>Папка с данным название уже существует. Удалите её или создайте с другим названием  

# Error_Глава 2
> Ошибки новичков:
> - на это этапе возможно будут опечатки
> - пропуск символов 
> - перепутать или заменять местами ; , или "" ''
> - закрытие { } " " 
> - println!("") есть !

---
```rust 
 PS D:\Coding\Rust\name> cargo run 
   Compiling name v0.1.0 (D:\Coding\Rust\name)
warning: unused variable: `name`
 --> src\main.rs:4:9
  |
4 |     let name = 1000;
  |         ^^^^ help: if this is intentional, prefix it with an underscore: `_name`   
  |
  = note: `#[warn(unused_variables)]` on by default

warning: `name` (bin "name") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.50s
     Running `target\debug\name.exe`
```
>Решиение: неиспользуется name на 4:9, если это сделано намерено то напешите _name
>
```
  |
4 |     let name = 1000;
  |         ^^^^ help: if this is intentional, prefix it with an underscore: `_name` 
```
---
