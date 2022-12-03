#include <cassert>

export module utils;

import std;

namespace utils {

    export using Lines = std::vector<std::string>;

    export [[nodiscard]] auto read_lines(const std::filesystem::path& filename) -> Lines {
        auto result = Lines{};
        auto file = std::ifstream{ filename };
        assert(static_cast<bool>(file));
        auto line = std::string{};
        while (std::getline(file, line)) {
            result.push_back(line);
        }
        return result;
    }

    export [[nodiscard]] auto to_usize(const std::string_view string) -> std::optional<usize> {
        auto value = usize{};
        const auto conversion_result = std::from_chars(&string.front(), &string.front() + string.length(), value);
        const auto success = (conversion_result.ec == std::errc{});
        if (success) {
            return value;
        }
        return {};
    }

} // namespace utils
