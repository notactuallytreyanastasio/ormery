namespace Ormery
{
    public class Field
    {
        readonly string name__252;
        readonly string fieldType__253;
        readonly bool primaryKey__254;
        readonly bool nullable__255;
        public string Description
        {
            get
            {
                string pk__258;
                if (this.primaryKey__254)
                {
                    pk__258 = " (PK)";
                }
                else
                {
                    pk__258 = "";
                }
                string null__259;
                if (this.nullable__255)
                {
                    null__259 = " (nullable)";
                }
                else
                {
                    null__259 = "";
                }
                return this.name__252 + ": " + this.fieldType__253 + pk__258 + null__259;
            }
        }
        public Field(string name__261, string fieldType__262, bool primaryKey__263, bool nullable__264)
        {
            this.name__252 = name__261;
            this.fieldType__253 = fieldType__262;
            this.primaryKey__254 = primaryKey__263;
            this.nullable__255 = nullable__264;
        }
        public string Name
        {
            get
            {
                return this.name__252;
            }
        }
        public string FieldType
        {
            get
            {
                return this.fieldType__253;
            }
        }
        public bool PrimaryKey
        {
            get
            {
                return this.primaryKey__254;
            }
        }
        public bool Nullable
        {
            get
            {
                return this.nullable__255;
            }
        }
    }
}
