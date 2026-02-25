using S = System;
using G = System.Collections.Generic;
using L = System.Linq;
using C = TemperLang.Core;
namespace Ormery
{
    public class Schema
    {
        readonly string tableName__265;
        readonly G::IReadOnlyList<Field> fields__266;
        public Field GetField(string name__268)
        {
            Field return__117;
            {
                {
                    G::IReadOnlyList<Field> this__2501 = this.fields__266;
                    int n__2502 = this__2501.Count;
                    int i__2503 = 0;
                    while (i__2503 < n__2502)
                    {
                        Field el__2504 = this__2501[i__2503];
                        i__2503 = i__2503 + 1;
                        Field field__270 = el__2504;
                        if (field__270.Name == name__268)
                        {
                            return__117 = field__270;
                            goto fn__269;
                        }
                    }
                    throw new S::Exception();
                }
                fn__269:
                {
                }
            }
            return return__117;
        }
        public bool HasField(string name__272)
        {
            bool return__118;
            {
                {
                    G::IReadOnlyList<Field> this__2506 = this.fields__266;
                    int n__2507 = this__2506.Count;
                    int i__2508 = 0;
                    while (i__2508 < n__2507)
                    {
                        Field el__2509 = this__2506[i__2508];
                        i__2508 = i__2508 + 1;
                        Field field__274 = el__2509;
                        if (field__274.Name == name__272)
                        {
                            return__118 = true;
                            goto fn__273;
                        }
                    }
                    return__118 = false;
                }
                fn__273:
                {
                }
            }
            return return__118;
        }
        public Field PrimaryKeyField
        {
            get
            {
                Field return__119;
                {
                    {
                        G::IReadOnlyList<Field> this__2511 = this.fields__266;
                        int n__2512 = this__2511.Count;
                        int i__2513 = 0;
                        while (i__2513 < n__2512)
                        {
                            Field el__2514 = this__2511[i__2513];
                            i__2513 = i__2513 + 1;
                            Field field__277 = el__2514;
                            if (field__277.PrimaryKey)
                            {
                                return__119 = field__277;
                                goto fn__276;
                            }
                        }
                        throw new S::Exception();
                    }
                    fn__276:
                    {
                    }
                }
                return return__119;
            }
        }
        public G::IReadOnlyList<string> FieldNames
        {
            get
            {
                string fn__4076(Field f__280)
                {
                    return f__280.Name;
                }
                return C::Listed.ToReadOnlyList(L::Enumerable.Select(this.fields__266, (S::Func<Field, string>) fn__4076));
            }
        }
        public string Describe()
        {
            string header__284 = "Schema: " + this.tableName__265 + "\n";
            string fn__4070(Field f__286)
            {
                return "  - " + f__286.Description;
            }
            G::IReadOnlyList<string> t___4072 = C::Listed.ToReadOnlyList(L::Enumerable.Select(this.fields__266, (S::Func<Field, string>) fn__4070));
            string fn__4069(string s__288)
            {
                return s__288;
            }
            string fieldList__285 = C::Listed.Join(t___4072, "\n", (S::Func<string, string>) fn__4069);
            return header__284 + fieldList__285;
        }
        public Schema(string tableName__291, G::IReadOnlyList<Field> fields__292)
        {
            this.tableName__265 = tableName__291;
            this.fields__266 = fields__292;
        }
        public string TableName
        {
            get
            {
                return this.tableName__265;
            }
        }
        public G::IReadOnlyList<Field> Fields
        {
            get
            {
                return this.fields__266;
            }
        }
    }
}
