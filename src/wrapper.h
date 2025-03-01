// This header contains all the required DynamoRIO headers 
// and is used to generate Rust bindings to the C types
#include <dr_api.h>
#include <dr_frontend.h>

#ifdef __FEATURE_BBDUP
#   include <drext.h>
#   include <drbbdup.h>
#endif
#ifdef __FEATURE_CONTAINERS
#   include <drext.h>
#   include <drtable.h>
#   include <drvector.h>
#   include <hashtable.h>
#endif
#ifdef __FEATURE_COVLIB
#   include <drext.h>
#   include <drcovlib.h>
#endif
#ifdef __FEATURE_MGR
#   include <drext.h>
#   include <drmgr.h>
#endif
#ifdef __FEATURE_OPTION
#   include <drext.h>
#   include <droption.h>
#endif
#ifdef __FEATURE_REG
#   include <drext.h>
#   include <drreg.h>
#endif
#ifdef __FEATURE_SYMS
#   include <drext.h>
#   include <drsyms.h>
#endif
#ifdef __FEATURE_UTIL
#   include <drext.h>
#   include <drutil.h>
#endif
#ifdef __FEATURE_WRAP
#   include <drext.h>
#   include <drwrap.h>
#endif
#ifdef __FEATURE_X
#   include <drext.h>
#   include <drx.h>
#endif
