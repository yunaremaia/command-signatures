#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_all_packages_filters_empty_lines() {
        let output = "vim\ngit\n\n";
        let results = list_all_packages(output);
        let suggestions: Vec<_> = results.into_iter().collect();
        assert_eq!(suggestions.len(), 2);
        assert_eq!(suggestions[0].name, "vim");
        assert_eq!(suggestions[1].name, "git");
    }

    #[test]
    fn test_list_all_packages_empty_input() {
        let results = list_all_packages("");
        let suggestions: Vec<_> = results.into_iter().collect();
        assert_eq!(suggestions.len(), 0);
    }

    #[test]
    fn test_list_all_deb_files_filters_empty_lines() {
        let output = "./foo.deb\n./bar.deb\n\n";
        let results = list_all_deb_files_in_cwd(output);
        let suggestions: Vec<_> = results.into_iter().collect();
        assert_eq!(suggestions.len(), 2);
        assert_eq!(suggestions[0].name, "./foo.deb");
        assert_eq!(suggestions[1].name, "./bar.deb");
    }

    #[test]
    fn test_list_all_deb_files_empty_input() {
        let results = list_all_deb_files_in_cwd("");
        let suggestions: Vec<_> = results.into_iter().collect();
        assert_eq!(suggestions.len(), 0);
    }
}
