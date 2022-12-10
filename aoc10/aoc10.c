#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>

typedef enum {
    MNEMONIC_ADDX,
    MNEMONIC_NOOP,
    MNEMONIC_UNKNOWN,
} Mnemonic;

Mnemonic parse_mnemonic(const char* const string) {
    if (strcmp(string, "addx") == 0) {
        return MNEMONIC_ADDX;
    }
    if (strcmp(string, "noop") == 0) {
        return MNEMONIC_NOOP;
    }
    return MNEMONIC_UNKNOWN;
}

bool is_relevant_cycle(const size_t cycle) {
    return cycle >= 20 && (cycle - 20) % 40 == 0;
}

void try_accumulate(const size_t cycle, const int register_value, int* result) {
    if (is_relevant_cycle(cycle)) {
        *result += (int)cycle * register_value;
    }
}

void draw_pixel(const size_t cycle, const int register_value) {
    const size_t x = (cycle - 1) % 40;
    const bool is_visible = abs((int)x - register_value) <= 1;
    printf("%c", is_visible ? '#' : ' ');
    if (x == 39) {
        printf("\n");
    }
}

int main(void) {
    FILE *file = fopen("real_input.txt", "r");
    if (file == NULL) {
        return EXIT_FAILURE;
    }

    size_t cycle = 1;
    int register_x = 1;
    int result = 0;
    char line[16];
    while (fgets(line, sizeof(line), file) != NULL) {
        const size_t length = strlen(line);
        line[length - 1] = '\0';
        const char* const mnemonic_string = strtok(line, " ");
        //printf("%s", mnemonic_string);

        int argument = 0;
        const char* const argument_string = strtok(NULL, " ");
        const bool has_argument = argument_string != NULL;
        if (has_argument) {
            argument = atoi(argument_string);
            //printf(" %d\n", argument);
        } else {
            //printf("\n");
        }
        const Mnemonic mnemonic = parse_mnemonic(mnemonic_string);
        assert(mnemonic != MNEMONIC_UNKNOWN);
        assert((mnemonic == MNEMONIC_NOOP && !has_argument) || (mnemonic == MNEMONIC_ADDX && has_argument));
        try_accumulate(cycle, register_x, &result);
        draw_pixel(cycle, register_x);
        ++cycle;
        switch (mnemonic) {
            case MNEMONIC_NOOP:
                break;
            case MNEMONIC_ADDX:
                try_accumulate(cycle, register_x, &result);
                draw_pixel(cycle, register_x);
                ++cycle;
                register_x += argument;
                break;
            default:
                assert(false && "unreachable");
                break;
        }
    }
    try_accumulate(cycle, register_x, &result);

    fclose(file);

    printf("result: %d\n", result);

    return EXIT_SUCCESS;
}
