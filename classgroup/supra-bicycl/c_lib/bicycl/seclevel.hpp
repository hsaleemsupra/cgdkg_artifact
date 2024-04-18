/*
 * BICYCL Implements CryptographY in CLass groups
 * Copyright (C) 2022  Cyril Bouvier <cyril.bouvier@lirmm.fr>
 *                     Guilhem Castagnos <guilhem.castagnos@math.u-bordeaux.fr>
 *                     Laurent Imbert <laurent.imbert@lirmm.fr>
 *                     Fabien Laguillaumie <fabien.laguillaumie@lirmm.fr>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */
#ifndef SECLEVEL_HPP__
#define SECLEVEL_HPP__

#include <iostream>
#include <stdexcept>
#include <string>
#include <vector>

namespace BICYCL
{
  class InvalidSecLevelException : public std::invalid_argument
  {
    public:
      InvalidSecLevelException() : std::invalid_argument("not a valid SecLevel") { }
  };

  class SecLevel
  {
    public:
      enum Value : unsigned int
      {
        sec_112 = 112,
        sec_128 = 128,
        sec_192 = 192,
        sec_256 = 256,
      };

      static const std::vector<SecLevel> All ()
      {
        return { sec_112, sec_128, sec_192, sec_256 };
      }

      SecLevel() = delete;
      constexpr SecLevel (Value seclevel) : value_(seclevel) { }
      SecLevel (unsigned int s)
      {
        switch(s)
        {
          case 112 : value_ = sec_112; break;
          case 128 : value_ = sec_128; break;
          case 192 : value_ = sec_192; break;
          case 256 : value_ = sec_256; break;
          default  : throw InvalidSecLevelException() ; break;
        }
      }

      SecLevel (const std::string &s)
      {
        if (s == "112")       value_ = sec_112;
        else if (s == "128")  value_ = sec_128;
        else if (s == "192")  value_ = sec_192;
        else if (s == "256")  value_ = sec_256;
        else                  throw InvalidSecLevelException();
      }

      /* Allow switch, comparisons and usage as key std::map */
      constexpr operator Value() const { return value_; }

      /* */
      explicit operator bool() = delete;

      /* */
      size_t RSA_modulus_bitsize () const
      {
        if (value_ == sec_112)        return 2048;
        else if (value_ == sec_128)   return 3072;
        else if (value_ == sec_192)   return 7680;
        else if (value_ == sec_256)   return 15360;
        else                       throw InvalidSecLevelException();
      }

      /* */
      size_t discriminant_bitsize () const
      {
        if (value_ == sec_112)        return 1348;
        else if (value_ == sec_128)   return 1827;
        else if (value_ == sec_192)   return 3598;
        else if (value_ == sec_256)   return 5971;
        else                       throw InvalidSecLevelException();
      }

      /* */
      /*friend std::ostream & operator< (std::ostream &o, SecLevel seclevel)
      {
        return o << static_cast<unsigned int>(seclevel.value_);
      }*/

    protected:
      Value value_;
  };
} /* BICYCL namespace */


#endif /* SECLEVEL_HPP__ */
