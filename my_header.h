#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

constexpr static const uintptr_t MATRIX_COUNT_ROWS = 4;

constexpr static const uintptr_t MATRIX_COUNT_COLS = 4;

enum class Direction {
  Row,
  Col,
};

using ElementMatrix = float;

struct PosElem {
  uintptr_t row;
  uintptr_t col;
};

struct Matrix {
  ElementMatrix content[MATRIX_COUNT_ROWS][MATRIX_COUNT_COLS];
  PosElem count;
  Direction direction;
};

extern "C" {

void hello_world();

Matrix init(PosElem count, float value);

bool is_square(const Matrix *self);

float determinant(const Matrix *self);

bool elem_is_exist(const Matrix *self, PosElem pos);

} // extern "C"
