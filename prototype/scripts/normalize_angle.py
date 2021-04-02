# // Normalizes any number to an arbitrary range
# // by assuming the range wraps around when going below min or above max
# double normalize( const double value, const double start, const double end )
# {
#  const double width       = end - start   ;   //
#  const double offsetValue = value - start ;   // value relative to 0
#
#  return ( offsetValue - ( floor( offsetValue / width ) * width ) ) + start ;
#  // + start to reset back to start of original range
# }

import math


def normalize(value, start, end):
    width = end - start
    off = value - start

    return (off - (math.floor(off / width) * width)) + start


print(normalize(-math.pi, math.pi, 1))

# Does not work for radians...
