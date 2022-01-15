#include "dr_api.h"

file_t dr_stdout(void)
{
	return STDOUT;
}

file_t dr_stderr(void)
{
	return STDERR;
}

file_t dr_stdin(void)
{
	return STDIN;
}
