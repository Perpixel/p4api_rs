#ifndef P4API_WRAPPER_H_
#define P4API_WRAPPER_H_

#include "p4/clientapi.h"
#include "p4/p4libs.h"

// Wrapper for stuff that bindgen doesn't support just yet

// Constructor inlined and not picked up by bindgen

class P4Error: public Error
{
public:
    P4Error();
    ~P4Error();
};


class P4StrBuf: public StrBuf
{
public:
    P4StrBuf();
    ~P4StrBuf();
};

#endif // P4API_WRAPPER_H_
