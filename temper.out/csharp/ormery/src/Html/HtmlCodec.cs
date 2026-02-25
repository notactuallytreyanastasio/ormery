using C0 = Ormery.Core;
using C1 = TemperLang.Core;
using H = Ormery.Html;
using T = System.Text;
namespace Ormery.Html
{
    public class HtmlCodec: C0::ICodec
    {
        public string Encode(string s__345)
        {
            string return__127;
            int t___5670;
            int t___5673;
            int t___5674;
            string t___3392;
            T::StringBuilder sb__347 = new T::StringBuilder();
            int end__348 = s__345.Length;
            int encodedTo__349 = 0;
            int i__350 = 0;
            while (i__350 < end__348)
            {
                {
                    {
                        t___5670 = C1::StringUtil.Get(s__345, i__350);
                        if (t___5670 == 38)
                        {
                            t___3392 = "&amp;";
                        }
                        else if (t___5670 == 60)
                        {
                            t___3392 = "&lt;";
                        }
                        else if (t___5670 == 62)
                        {
                            t___3392 = "&gt;";
                        }
                        else if (t___5670 == 39)
                        {
                            t___3392 = "&#39;";
                        }
                        else if (t___5670 == 34)
                        {
                            t___3392 = "&#34;";
                        }
                        else if (t___5670 == 0)
                        {
                            t___3392 = "&#0;";
                        }
                        else goto continue___5752;
                        string replacement__351 = t___3392;
                        C1::StringUtil.AppendBetween(sb__347, s__345, encodedTo__349, i__350);
                        sb__347.Append(replacement__351);
                        t___5673 = C1::StringUtil.Next(s__345, i__350);
                        encodedTo__349 = t___5673;
                    }
                    continue___5752:
                    {
                    }
                }
                t___5674 = C1::StringUtil.Next(s__345, i__350);
                i__350 = t___5674;
            }
            if (encodedTo__349 > 0)
            {
                C1::StringUtil.AppendBetween(sb__347, s__345, encodedTo__349, end__348);
                return__127 = sb__347.ToString();
            }
            else
            {
                return__127 = s__345;
            }
            return return__127;
        }
        public string Decode(string s__353)
        {
            string return__128;
            int t___5647;
            int t___5648;
            int t___5650;
            int t___5652;
            int t___5653;
            int t___5654;
            int t___5659;
            int t___5665;
            bool t___3360;
            bool t___3366;
            bool t___3367;
            bool t___3370;
            int t___3373;
            bool t___3375;
            bool t___3376;
            bool t___3380;
            T::StringBuilder sb__355 = new T::StringBuilder();
            int end__356 = s__353.Length;
            int decodedTo__357 = 0;
            int i__358 = 0;
            while (i__358 < end__356)
            {
                {
                    if (C1::StringUtil.Get(s__353, i__358) == 38)
                    {
                        int startOfEntity__359 = C1::StringUtil.Next(s__353, i__358);
                        int endOfEntity__360 = startOfEntity__359;
                        if (startOfEntity__359 < end__356)
                        {
                            t___5647 = C1::StringUtil.Get(s__353, startOfEntity__359);
                            t___3360 = t___5647 == "#";
                        }
                        else
                        {
                            t___3360 = false;
                        }
                        if (t___3360)
                        {
                            t___5648 = C1::StringUtil.Next(s__353, startOfEntity__359);
                            endOfEntity__360 = t___5648;
                            if (endOfEntity__360 >= end__356) goto continue___5753;
                            int base__361 = 10;
                            if ((C1::StringUtil.Get(s__353, endOfEntity__360) | 32) == 120)
                            {
                                t___5650 = C1::StringUtil.Next(s__353, endOfEntity__360);
                                endOfEntity__360 = t___5650;
                                base__361 = 16;
                            }
                            int digitQuota__362 = 7;
                            int startOfDigits__363 = endOfEntity__360;
                            while (true)
                            {
                                if (!(endOfEntity__360 < end__356)) break;
                                int cp__364 = C1::StringUtil.Get(s__353, endOfEntity__360);
                                if (48 <= cp__364)
                                {
                                    t___3366 = cp__364 <= 57;
                                }
                                else
                                {
                                    t___3366 = false;
                                }
                                if (!t___3366) if (base__361 == 16)
                                {
                                    int lcp__365 = cp__364 | 32;
                                    if (97 <= lcp__365)
                                    {
                                        t___3367 = lcp__365 <= 102;
                                    }
                                    else
                                    {
                                        t___3367 = false;
                                    }
                                    if (!t___3367) break;
                                }
                                else break;
                                t___5652 = C1::StringUtil.Next(s__353, endOfEntity__360);
                                endOfEntity__360 = t___5652;
                            }
                            int endOfDigits__366 = endOfEntity__360;
                            if (endOfDigits__366 == startOfDigits__363) goto continue___5753;
                            if (endOfEntity__360 < end__356)
                            {
                                t___5653 = C1::StringUtil.Get(s__353, endOfEntity__360);
                                t___3370 = t___5653 == 59;
                            }
                            else
                            {
                                t___3370 = false;
                            }
                            if (t___3370)
                            {
                                t___5654 = C1::StringUtil.Next(s__353, endOfEntity__360);
                                endOfEntity__360 = t___5654;
                            }
                            try
                            {
                                t___3373 = C1::Core.ToInt(C1::StringUtil.Slice(s__353, startOfDigits__363, endOfDigits__366), base__361);
                            }
                            catch
                            {
                                goto continue___5753;
                            }
                            int decodedCp__367 = t___3373;
                            if (0 <= decodedCp__367)
                            {
                                t___3375 = decodedCp__367 <= 1114111;
                            }
                            else
                            {
                                t___3375 = false;
                            }
                            if (t___3375)
                            {
                                C1::StringUtil.AppendBetween(sb__355, s__353, decodedTo__357, i__358);
                                if (55296 <= decodedCp__367)
                                {
                                    t___3376 = decodedCp__367 <= 57343;
                                }
                                else
                                {
                                    t___3376 = false;
                                }
                                if (t___3376) sb__355.Append("�");
                                else
                                {
                                    try
                                    {
                                        C1::StringUtil.AppendCodePoint(sb__355, decodedCp__367);
                                    }
                                    catch
                                    {
                                        goto continue___5753;
                                    }
                                }
                                decodedTo__357 = endOfEntity__360;
                            }
                        }
                        else
                        {
                            while (endOfEntity__360 < end__356)
                            {
                                int cp__368 = C1::StringUtil.Get(s__353, endOfEntity__360);
                                t___5659 = C1::StringUtil.Next(s__353, endOfEntity__360);
                                endOfEntity__360 = t___5659;
                                if (cp__368 == 59) break;
                                int lcp__369 = cp__368 | 32;
                                if (97 <= lcp__369)
                                {
                                    t___3380 = lcp__369 <= 122;
                                }
                                else
                                {
                                    t___3380 = false;
                                }
                                if (!t___3380) break;
                            }
                            if (startOfEntity__359 < endOfEntity__360)
                            {
                                string entityName__370 = C1::StringUtil.Slice(s__353, startOfEntity__359, endOfEntity__360);
                                string entityValue__371 = C1::Mapped.GetOrDefault(H::HtmlGlobal.htmlNamedCharacters__267, entityName__370, "");
                                if (!string.IsNullOrEmpty(entityValue__371))
                                {
                                    C1::StringUtil.AppendBetween(sb__355, s__353, decodedTo__357, i__358);
                                    sb__355.Append(entityValue__371);
                                    decodedTo__357 = endOfEntity__360;
                                }
                            }
                        }
                    }
                    continue___5753:
                    {
                    }
                }
                t___5665 = C1::StringUtil.Next(s__353, i__358);
                i__358 = t___5665;
            }
            if (decodedTo__357 > 0)
            {
                C1::StringUtil.AppendBetween(sb__355, s__353, decodedTo__357, end__356);
                return__128 = sb__355.ToString();
            }
            else
            {
                return__128 = s__353;
            }
            return return__128;
        }
        public HtmlCodec()
        {
        }
    }
}
