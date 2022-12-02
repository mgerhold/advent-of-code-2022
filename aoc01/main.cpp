#include <cassert>

import std;
import utils;

auto main() -> int {
    const auto filename = std::filesystem::path{ "real_input.txt" };
    const auto lines = read_lines(filename);
    auto elf_calories = std::vector<usize>{};
    auto accumulator = usize{ 0 };
    const auto flush = [&elf_calories, &accumulator]() {
        elf_calories.push_back(accumulator);
        accumulator = 0;
    };
    for (const auto& line : lines) {
        if (line.empty()) {
            flush();
            continue;
        }
        const auto calories = to_usize(line);
        assert(calories.has_value());
        accumulator += *calories;
    }
    if (accumulator > 0) {
        flush();
    }

    const auto max = std::ranges::max_element(elf_calories);
    assert(max != elf_calories.cend());
    std::cout << std::format("The elf with the maximum amount of calories carries {} calories.\n", *max);

    // part 2
    auto top_three = std::array<std::optional<usize>, 3>{};
    for (const auto calories : elf_calories) {
        for (usize i = 0; i < top_three.size(); ++i) {
            auto& entry = top_three[i];
            if (not entry.has_value()) {
                entry = calories;
                break;
            }
            if (calories >= *entry) {
                for (usize j = top_three.size() - 1; j > i; --j) {
                    top_three[j] = top_three[j - 1];
                }
                *entry = calories;
                break;
            }
        }
    }

    std::cout << "~~~~~~~\n";
    for (const auto& calories : top_three) {
        if (calories.has_value()) {
            std::cout << *calories;
        } else {
            std::cout << "-";
        };
        std::cout << "\n";
    }

    const auto sum = [&]() {
        auto result = usize{};
        for (const auto& calories : top_three) {
            result += *calories;
        }
        return result;
    }(); // <- immediately invoked

    std::cout << "sum of top three: " << sum << "\n";
}
