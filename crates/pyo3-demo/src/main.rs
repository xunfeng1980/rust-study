fn main() {}


#[cfg(test)]
mod tests {
    // 注意这个惯用法：在 tests 模块中，从外部作用域导入所有名字。
    use pyo3::prelude::*;
    use pyo3::types::{IntoPyDict, PyDictValues, PyTuple};

    #[test]
    fn test_invoke_python_fn() {
        let arg1 = "arg1";
        let arg2 = "arg2";
        let arg3 = "arg3";

        Python::with_gil(|py| {
            let fun: Py<PyAny> = PyModule::from_code(
                py,
                "def example(*args, **kwargs):
                if args != ():
                    print('called with args', args)
                if kwargs != {}:
                    print('called with kwargs', kwargs)
                if args == () and kwargs == {}:
                    print('called with no arguments')",
                "",
                "",
            ).expect("").getattr("example").expect("").into();

            // call object without any arguments
            fun.call0(py).expect("call python function failed");

            // call object with PyTuple
            let args = PyTuple::new(py, &[arg1, arg2, arg3]);
            fun.call1(py, args).expect("call python function failed");

            // pass arguments as rust tuple
            let args = (arg1, arg2, arg3);
            fun.call1(py, args).expect("call python function failed");
            println!("print form rust");
        });
    }

    #[test]
    fn test_print_py_version()
    {
        Python::with_gil(|py| {
            let sys = py.import("sys").unwrap();
            let version: String = sys.getattr("version").unwrap().extract().unwrap();

            let locals = [("os", py.import("os").unwrap())].into_py_dict(py);
            let code = "os.getenv('USER') or os.getenv('USERNAME') or 'Unknown'";
            let user: String = py.eval(code, None, Some(&locals)).unwrap().extract().unwrap();

            println!("Hello {}, I'm Python {}", user, version)
        });
    }

    #[test]
    fn test_call_transformers() {
        Python::with_gil(|py| {
            let fun = PyModule::from_code(
                py,
                r#"from transformers import pipeline
unmasker = pipeline('fill-mask', model='albert-base-v2')"#,
                "",
                "",
            ).unwrap();

            let args = ("Hello I'm a [MASK] model.", );
            let res: &PyAny = fun.getattr("unmasker").unwrap().call1(args).unwrap();
            println!("{:?}", res);
        });
    }
}


