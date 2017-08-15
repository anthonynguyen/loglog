error_chain!{
    foreign_links {
        SetLogger(::log::SetLoggerError);
    }
}
