#ifndef _ZEC_PAPER_RUST_H
#define _ZEC_PAPER_RUST_H

#ifdef __cplusplus
extern "C"{
#endif

extern char * rust_generate_wallet(bool testnet, unsigned int count, const char* entropy);
extern void   rust_free_string(char* s);

#ifdef __cplusplus
}
#endif
#endif