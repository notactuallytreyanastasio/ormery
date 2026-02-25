namespace Ormery.Core
{
    public class CollectedPart<PART__8>: ICollected<PART__8>
    {
        readonly PART__8 part__98;
        public CollectedPart(PART__8 part__100)
        {
            this.part__98 = part__100;
        }
        public PART__8 Part
        {
            get
            {
                return this.part__98;
            }
        }
    }
}
