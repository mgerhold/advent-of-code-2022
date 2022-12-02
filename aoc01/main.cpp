#include <cassert>

import std;
import utils;

[[nodiscard]] auto elf_calories_from_lines(const std::vector<std::string>& lines) {
    auto result = std::vector<usize>{};
    auto accumulator = usize{ 0 };

    const auto flush = [&result, &accumulator]() {
        result.push_back(accumulator);
        accumulator = 0;
    };

    for (const auto& line : lines) {
        if (line.empty()) {
            flush();
            continue;
        }
        const auto calories = utils::to_usize(line);
        assert(calories.has_value());
        accumulator += *calories;
    }

    if (accumulator > 0) {
        flush();
    }

    return result;
}

template<usize num_bests>
[[nodiscard]] auto get_bests(const std::vector<usize>& elf_calories) {
    assert(elf_calories.size() >= num_bests);

    auto result = std::array<std::optional<usize>, num_bests>{};

    for (const auto calories : elf_calories) {
        for (usize i = 0; i < result.size(); ++i) {
            auto& entry = result[i];

            if (not entry.has_value()) {
                entry = calories;
                break;
            }

            if (calories >= *entry) {
                for (usize j = result.size() - 1; j > i; --j) {
                    result[j] = result[j - 1];
                }
                *entry = calories;
                break;
            }
        }
    }

    return result;
}

auto main() -> int {
    // part 1
    const auto lines = utils::read_lines("real_input.txt");
    const auto elf_calories = elf_calories_from_lines(lines);

    const auto max = std::ranges::max_element(elf_calories);
    assert(max != elf_calories.cend());
    std::cout << std::format("The elf with the maximum amount of calories carries {} calories.\n", *max);

    std::cout << "~~~~~~~\n";

    // part 2
    static constexpr auto num_bests = 3; // we want the top three elves

    assert(num_bests <= elf_calories.size());

    const auto bests = get_bests<num_bests>(elf_calories);
    std::cout << std::format("top {} elves:\n", num_bests);

    std::ranges::for_each(bests, [](const auto& best) {
        assert(best.has_value());
        std::cout << std::format("{}\n", *best);
    });

    const auto calories_range = std::ranges::transform_view(bests, [](const auto& calories) { return *calories; });
    const auto sum = std::accumulate(calories_range.begin(), calories_range.end(), usize{ 0 });
    std::cout << std::format("sum of top three: {}\n", sum);
}
