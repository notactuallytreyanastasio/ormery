using T = System.Text;
namespace Ormery.Sql
{
    public class SqlBoolean: ISqlPart
    {
        readonly bool value__179;
        public void FormatTo(T::StringBuilder builder__181)
        {
            string t___718;
            if (this.value__179)
            {
                t___718 = "TRUE";
            }
            else
            {
                t___718 = "FALSE";
            }
            builder__181.Append(t___718);
        }
        public SqlBoolean(bool value__184)
        {
            this.value__179 = value__184;
        }
        public bool Value
        {
            get
            {
                return this.value__179;
            }
        }
    }
}
