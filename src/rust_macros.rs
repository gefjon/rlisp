#[macro_export]
macro_rules! pop_bubble {
    ($l:expr) => {
        {
            let obj = <_ as $crate::lisp::stack_storage::Stack>
                ::pop($l);
            bubble!(obj);
            obj
        }
    }
}

#[macro_export]
macro_rules! bubble {
    ($obj:expr) => {
        if <& $crate::types::RlispError as $crate::types::conversions::FromObject>
            ::is_type($obj) {
                return $obj;
            }
    }
}

#[macro_export]
macro_rules! try_rlisp_err {
    ($l:ident : $result:expr) => {
        match $result {
            Err(err) => {
                return <_ as $crate::lisp::allocate::AllocObject>
                    ::alloc($l, <$crate::types::RlispError>::from(err));
            }
            Ok(res) => res,
        }
    }
}

#[macro_export]
macro_rules! into_type_or_error {
    ($l:ident : $obj:expr => $type:ty) => {
        if let Some(val) =
            <$type as $crate::types::conversions::MaybeFrom<Object>>
            ::maybe_from($obj) {
            val
        } else {

            let wanted_type = <_ as $crate::lisp::symbols_table::Symbols>
                ::type_name($l, <$type as $crate::types::conversions::FromObject>::rlisp_type());
            let got_type = <_ as $crate::lisp::symbols_table::Symbols>
                ::type_name($l, $obj.what_type());
            let e = RlispError::wrong_type(
                wanted_type,
                got_type
            );
            return <_ as $crate::lisp::allocate::AllocObject>
                ::alloc($l, e);
        }
    }
}

#[macro_export]
macro_rules! into_type_or_result_error {
    ($l:ident : $obj:expr => $type:ty) => {
        if let Some(val) =
            <$type as $crate::types::conversions::MaybeFrom<Object>>
            ::maybe_from($obj) {
            val
        } else {
            let wanted_type = <_ as $crate::lisp::symbols_table::Symbols>
                ::type_name($l, <$type as $crate::types::conversions::FromObject>::rlisp_type());
            let got_type = <_ as $crate::lisp::symbols_table::Symbols>
                ::type_name($l, $obj.what_type());
            return Ok(<_ as $crate::lisp::allocate::AllocObject>
                ::alloc($l, RlispError::wrong_type(
                wanted_type,
                got_type
            )));
        }
    }
}

#[macro_export]
macro_rules! into_type_or_result_option_error {
    ($l:ident : $obj:expr => $type:ty) => {
        if let Some(val) =
            <$type as $crate::types::conversions::MaybeFrom<Object>>
            ::maybe_from($obj) {
            val
        } else {
            let wanted_type = <_ as $crate::lisp::symbols_table::Symbols>
                ::type_name($l, <$type as $crate::types::conversions::FromObject>::rlisp_type());
            let got_type = <_ as $crate::lisp::symbols_table::Symbols>
                ::type_name($l, $obj.what_type());
            return Ok(Some(
                <_ as $crate::lisp::allocate::AllocObject>
                ::alloc($l, RlispError::wrong_type(
                wanted_type,
                got_type
            ))));
        }
    }
}

#[macro_export]
macro_rules! into_type_or_option_error {
    ($l:ident : $obj:expr => $type:ty) => {
        if let Some(val) =
            <$type as $crate::types::conversions::MaybeFrom<Object>>
            ::maybe_from($obj) {
            val
        } else {
            let wanted_type = <_ as $crate::lisp::symbols_table::Symbols>
                ::type_name($l, <$type as $crate::types::conversions::FromObject>::rlisp_type());
            let got_type = <_ as $crate::lisp::symbols_table::Symbols>
                ::type_name($l, $obj.what_type());
            return Some(
                <_ as $crate::lisp::allocate::AllocObject>
                ::alloc($l, RlispError::wrong_type(
                wanted_type,
                got_type
            )));
        }
    }
}

macro_rules! arg_in_list {
    ($arg:ident) => {
        String::from(stringify!($arg))
    };
    (($arg:ident $_ty:ty)) => {
        String::from(stringify!($arg))
    };
}

macro_rules! arglist {
    ($($arg:ident)* &optional $($oarg:tt)+) => {
        {
            let mut arglist = Vec::new();
            $(arglist.push(arg_in_list!($arg));)*;
            arglist.push(String::from("&optional"));
            $(arglist.push(arg_in_list!($oarg));)+;
            arglist
        }
    };
    ($($arg:ident)* &rest $($rarg:tt)+) => {
        // Only special forms get to have multiple `&rest` args
        {
            let mut arglist = Vec::new();
            $(arglist.push(arg_in_list!($arg));)*;
            arglist.push(String::from("&rest"));
            $(arglist.push(arg_in_list!($rarg));)+;
            arglist
        }
    };
    ($($arg:ident)*) => {
        {
            let mut arglist = Vec::new();
            $(arglist.push(arg_in_list!($arg));)*;
            arglist
        }
    };
}

macro_rules! get_arg {
    ($l:ident ; $n_args:ident ; $consumed_args:ident ; $arg:ident) => {
        debug_assert!($consumed_args < $n_args);
        let $arg = pop_bubble!($l);
        $consumed_args += 1;

        debug!("popped {} as an argument; have now consumed {} args", $arg, $consumed_args);
    };

    (OPT $l:ident ; $n_args:ident ; $consumed_args:ident ; $arg:ident) => {
        let $arg = if $consumed_args < $n_args {
            $consumed_args += 1;
            pop_bubble!($l)
        } else {
            $crate::types::Object::nil()
        };
        debug!("popped {} as an argument; have now consumed {} args", $arg, $consumed_args);
    };

    (REST $l:ident ; $n_args:ident ; $consumed_args:ident ; $arg:ident) => {
        let $arg = {
            let mut head = $crate::types::Object::nil();
            while $consumed_args < $n_args {
                $consumed_args += 1;
                let conscell = $crate::types::ConsCell::new(
                    pop_bubble!($l), head);
                head = <$crate::lisp::Lisp as $crate::lisp::allocate::AllocObject>
                    ::alloc::<$crate::types::ConsCell>($l, conscell);
            }
            if head != $crate::types::Object::nil() {
                let head = into_type_or_error!($l : head => &ConsCell);
                <$crate::lisp::Lisp as $crate::list::ListOps>
                    ::list_reverse($l, head)
            } else {
                head
            }
        };
        debug!("popped {} as an argument; have now consumed {} args", $arg, $consumed_args);
    };
}

macro_rules! get_args {
    ($l:ident ; $n_args:ident ; $($arg:ident)*) => {
        let mut _consumed_args = 0;
        $(
            get_arg!($l ; $n_args ; _consumed_args ; $arg);
        )*;
    };

    ($l:ident ; $n_args:ident ; $($arg:ident)* &optional $($oarg:ident)+) => {
        let mut _consumed_args = 0;
        $(
            get_arg!($l ; $n_args ; _consumed_args ; $arg);
        )*;
        $(
            get_arg!(OPT $l ; $n_args ; _consumed_args ; $oarg);
        )+;
    };

    ($l:ident ; $n_args:ident ; $($arg:ident)* &rest $rarg:ident) => {
        let mut _consumed_args = 0;
        $(
            get_arg!($l ; $n_args ; _consumed_args ; $arg);
        )*;
        get_arg!(REST $l ; $n_args ; _consumed_args ; $rarg);
    };
}

macro_rules! builtin_function {
    ($l:ident ; $name:expr ; ($($arg:tt)*) -> $blk:block) => {
        {
            {
              (
                String::from($name),
                arglist!($($arg )*),
                Box::new(move |$l, _n_args| {
                    get_args!($l ; _n_args ; $($arg)*);
                    $blk
                })
             )
            }
        }
    };
}

macro_rules! special_form {
    ($l:ident ; $name:expr ; ($($arg:tt)*) -> $blk:block) => {
        {
            {
                (
                    String::from($name),
                    arglist!($($arg)*),
                    Box::new(move |$l| {
                        $blk
                    })
                )
            }
        }
    };
}

#[macro_export]
macro_rules! builtin_functions {
    (
        $l:tt = lisp;
        $($name:tt ($($arg:tt)*) -> $blk:block),* $(,)*
    ) => {{
        let mut v: $crate::builtins::RlispBuiltins = Vec::new();
        $(v.push(builtin_function!{$l ; $name ; ($($arg)*) -> $blk});)*;
        v
    }};
}

#[macro_export]
macro_rules! special_forms {
    (
        $l:tt = lisp;
        $($name:tt ($($arg:tt)*) -> $blk:block),* $(,)*
    ) => {{
        let mut v: $crate::builtins::RlispSpecialForms = Vec::new();
        $(v.push(special_form!{$l ; $name ; ($($arg)*) -> $blk});)*;
        v
    }};
}

#[macro_export]
macro_rules! builtin_vars {
    (
        $($name:tt = $val:expr),* $(,)*
    ) => {{
        let mut v: $crate::builtins::RlispBuiltinVars = Vec::new();
        $(v.push((String::from($name), $crate::types::into_object::IntoObject::from($val)));)*;
        v
    }};
}
