#include "expected.hpp"
#include "magic_enum.hpp"
#include "strong_type.hpp"
#include <cassert>

import std;
import utils;

using Score = strong::type<u64, struct Score_, strong::bicrementable, strong::difference, strong::formattable>;

enum class Shape : strong::underlying_type_t<Score> {
    // the values of the enum variants represent the score
    Rock = 1,
    Paper = 2,
    Scissors = 3,
};

[[nodiscard]] constexpr auto score(const Shape shape) {
    return Score{ std::to_underlying(shape) };
}

enum class ShapeConversionError {
    InvalidLetter,
};

[[nodiscard]] constexpr auto shape(const char letter) -> tl::expected<Shape, ShapeConversionError> {
    switch (letter) {
        case 'A':
        case 'X':
            return Shape::Rock;
        case 'B':
        case 'Y':
            return Shape::Paper;
        case 'C':
        case 'Z':
            return Shape::Scissors;
        default:
            return tl::unexpected{ ShapeConversionError::InvalidLetter };
    }
}

struct Round {
    enum class Outcome {
        Win,
        Lose,
        Draw,
    };

    Shape action;
    Shape response;

    [[nodiscard]] constexpr auto outcome() const {
        const auto outcome = std::ranges::find_if(outcomes, [=](const auto& tuple) {
            return std::get<0>(tuple) == action and std::get<1>(tuple) == response;
        });
        assert(outcome != outcomes.cend());
        return std::get<2>(*outcome);
    }

    static constexpr auto outcomes = std::array{
        std::tuple{    Shape::Rock,     Shape::Rock, Outcome::Draw},
        std::tuple{    Shape::Rock,    Shape::Paper,  Outcome::Win},
        std::tuple{    Shape::Rock, Shape::Scissors, Outcome::Lose},
        std::tuple{   Shape::Paper,     Shape::Rock, Outcome::Lose},
        std::tuple{   Shape::Paper,    Shape::Paper, Outcome::Draw},
        std::tuple{   Shape::Paper, Shape::Scissors,  Outcome::Win},
        std::tuple{Shape::Scissors,     Shape::Rock,  Outcome::Win},
        std::tuple{Shape::Scissors,    Shape::Paper, Outcome::Lose},
        std::tuple{Shape::Scissors, Shape::Scissors, Outcome::Draw},
    };
};

[[nodiscard]] auto score(const Round::Outcome outcome) {
    using enum Round::Outcome;
    switch (outcome) {
        case Lose:
            return Score{ 0 };
        case Draw:
            return Score{ 3 };
        case Win:
            return Score{ 6 };
    }
    std::unreachable();
}

using Rounds = std::vector<Round>;

enum class Part {
    Part1 = 1,
    Part2 = 2,
};

enum class OutcomeConversionError {
    InvalidLetter,
};

[[nodiscard]] constexpr auto outcome(const char letter) -> tl::expected<Round::Outcome, OutcomeConversionError> {
    using enum Round::Outcome;
    switch (letter) {
        case 'X':
            return Lose;
        case 'Y':
            return Draw;
        case 'Z':
            return Win;
    }
    return tl::unexpected{ OutcomeConversionError::InvalidLetter };
}

[[nodiscard]] constexpr auto response(const Shape action, const Round::Outcome desired_outcome) {
    const auto response = std::ranges::find_if(Round::outcomes, [&](const auto& tuple) {
        return std::get<0>(tuple) == action and std::get<2>(tuple) == desired_outcome;
    });
    assert(response != Round::outcomes.cend());
    return std::get<1>(*response);
}

template<Part part>
[[nodiscard]] auto parse_rounds(const utils::Lines& lines) {
    using enum Part;
    auto rounds = Rounds{};
    rounds.reserve(lines.size());
    for (const auto& line : lines) {
        assert(line.length() == 3);
        const auto round = [&]() {
            if constexpr (part == Part1) {
                return Round{ shape(line.front()).value(), shape(line.back()).value() };
            } else {
                static_assert(part == Part2);
                const auto opponent_action = shape(line.front()).value();
                const auto desired_outcome = outcome(line.back()).value();
                const auto needed_response = response(opponent_action, desired_outcome);
                return Round{ opponent_action, needed_response };
            }
        }();
        rounds.push_back(round);
    }
    return rounds;
}

[[nodiscard]] auto calculate_score(const Round& round) {
    return score(round.response) + score(round.outcome());
}

template<Part part>
auto print_single_result(const utils::Lines& lines) {
    const auto rounds = parse_rounds<part>(lines);
    const auto scores = std::ranges::views::transform(rounds, [](const auto& round) { return calculate_score(round); });
    const auto total_score = std::accumulate(scores.begin(), scores.end(), Score{ 0 });
    std::cout << std::format("part {}: total score: {}\n", std::to_underlying(part), total_score);
}

template<Part... parts>
auto print_results(const utils::Lines& lines) {
    (print_single_result<parts>(lines), ...);
}

auto main() -> int {
    using enum Part;
    const auto lines = utils::read_lines("real_input.txt");
    print_results<Part1, Part2>(lines);
}
