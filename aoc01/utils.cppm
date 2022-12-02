#include <cassert>

export module utils;

import std;

namespace utils {

    export [[nodiscard]] auto read_lines(const std::filesystem::path& filename) -> std::vector<std::string> {
        auto result = std::vector<std::string>{};
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
