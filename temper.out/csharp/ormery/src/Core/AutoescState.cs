namespace Ormery.Core
{
    public class AutoescState<CONTEXT__10> where CONTEXT__10: IContext
    {
        readonly CONTEXT__10 context__103;
        readonly Subsidiary ? subsidiary__104;
        public AutoescState(CONTEXT__10 context__106, Subsidiary ? subsidiary__107)
        {
            this.context__103 = context__106;
            this.subsidiary__104 = subsidiary__107;
        }
        public CONTEXT__10 Context
        {
            get
            {
                return this.context__103;
            }
        }
        public Subsidiary ? Subsidiary
        {
            get
            {
                return this.subsidiary__104;
            }
        }
    }
}
