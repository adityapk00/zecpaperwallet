#ifndef _ZEC_PAPER_RUST_H
#define _ZEC_PAPER_RUST_H

#ifdef __cplusplus
extern "C"{
#endif

extern char * rust_generate_wallet(bool testnet, unsigned int count);
extern void   rust_free_string(char * s);
extern void   rust_save_to_pdf(const char* addr, const char* filename);

#ifdef __cplusplus
}
#endif
#endif