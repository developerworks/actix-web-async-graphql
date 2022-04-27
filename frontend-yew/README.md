```shell
Adding yew v0.19.3 to dependencies.
    Features:
    - doc_test
    - wasm_bench
    - wasm_test
Adding wasm-bindgen v0.2.80 to dependencies.
    Features:
    + spans
    + std
    + wasm-bindgen-macro/spans
    - enable-interning
    - nightly
    - serde
    - serde-serialize
    - serde_json
    - strict-macro
    - xxx_debug_only_print_generated_code
Adding yew-router v0.16.0 to dependencies.
    Features:
    - wasm_test     
```

# yew-router 抽丝剥茧

## 安装依赖

```shell
cargo add yew-router
```

## 第一步: 程序入口

这里暂时不需要任何依赖, 后续有依赖模块时, 为了方清晰的了解每一步增加的代码需要哪些依赖模块, 我会直接添加到该步骤的示例代码中, 实际开发的时候, 应该把所有依赖模块放在文件顶部.

```rust
// 程序入口
fn main() {
    yew::Renderer::<App>::new().render();
}
```

## 创建根组件

```rust
// 根组件
// Switch::render 函数通过其参数 switch 回调函数来判断渲染哪一个组件
#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}
```