local temper = require('temper-core');
local appendHex__3, percentEscapeOctetTo, exports;
appendHex__3 = function(n__7, sb__8)
  local t_117, i__10, local_118, local_119, local_120;
  i__10 = temper.band(n__7, 15);
  local_118, local_119, local_120 = temper.pcall(function()
    if (i__10 < 10) then
      t_117 = 48;
    else
      t_117 = 87;
    end
    temper.stringbuilder_appendcodepoint(sb__8, temper.int32_add(i__10, t_117));
  end);
  if local_118 then
  else
    temper.bubble();
  end
  return nil;
end;
percentEscapeOctetTo = function(octet__4, sb__5)
  local t_122, t_123;
  temper.stringbuilder_append(sb__5, '%');
  t_122 = temper.int32_div(temper.band(octet__4, 255), 16);
  appendHex__3(t_122, sb__5);
  t_123 = temper.band(octet__4, 15);
  appendHex__3(t_123, sb__5);
  return nil;
end;
exports = {};
exports.percentEscapeOctetTo = percentEscapeOctetTo;
return exports;
