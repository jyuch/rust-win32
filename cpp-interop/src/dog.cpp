#include "dog.hpp"
#include <cstdio>

Dog::Dog(const char *name) :
    _name (name), _status (STATUS_STOP)
{
    std::printf("%s:new\n", _name);
}

Dog::~Dog()
{
    std::printf("%s:destruct\n", _name);
}

void Dog::walk()
{
    std::printf("%s:walk\n", _name);
    if (_status != STATUS_WALKING) _status = STATUS_WALKING;
}

void Dog::stop()
{
    std::printf("%s:stop\n", _name);
    if (_status != STATUS_STOP) _status = STATUS_STOP;
}