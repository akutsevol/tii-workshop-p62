#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct Aes128KeySchedule {
  __m128i rounds[NUM_ROUNDS];
};

extern "C" {

Aes128KeySchedule *aes128_load_key(const uint8_t *key);

void aes128_encode(const uint8_t *plain_text, uint8_t *cipher_text, const __m128i *key_schedule);

void aes128_decode(const uint8_t *cipher_text, uint8_t *plain_text, const __m128i *key_schedule);

void aes128_free_key_schedule(Aes128KeySchedule *schedule);

} // extern "C"
