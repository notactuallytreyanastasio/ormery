using C = Ormery.Core;
namespace Ormery.Core
{
    public class AfterPropagate<CONTEXT__11> where CONTEXT__11: IContext
    {
        readonly string adjustedString__108;
        readonly int consumed__109;
        readonly AutoescState<CONTEXT__11> stateAfter__110;
        public AfterPropagate<CONTEXT__11> Push(IDelegate delegate__112, ICodec codec__113)
        {
            return new AfterPropagate<CONTEXT__11>(this.adjustedString__108, this.consumed__109, new AutoescState<CONTEXT__11>(this.stateAfter__110.Context, new Subsidiary(delegate__112, codec__113)));
        }
        public AfterPropagate<CONTEXT__11> Pop()
        {
            return new AfterPropagate<CONTEXT__11>(this.adjustedString__108, this.consumed__109, new AutoescState<CONTEXT__11>(this.stateAfter__110.Context, null));
        }
        public AfterPropagate<CONTEXT__11> Feed(bool prepareForInterp__118)
        {
            AfterPropagate<CONTEXT__11> return__57;
            Subsidiary ? subsidiary__120 = this.stateAfter__110.Subsidiary;
            if (!(subsidiary__120 == null))
            {
                Subsidiary subsidiary___237 = subsidiary__120!;
                string adjustedFromDelegate__121 = C::CoreGlobal.feedSubsidiary__83(subsidiary___237, this.adjustedString__108, prepareForInterp__118);
                return__57 = new AfterPropagate<CONTEXT__11>(adjustedFromDelegate__121, this.consumed__109, this.stateAfter__110);
            }
            else
            {
                return__57 = this;
            }
            return return__57;
        }
        public AfterPropagate(string adjustedString__123, int consumed__124, AutoescState<CONTEXT__11> stateAfter__125)
        {
            this.adjustedString__108 = adjustedString__123;
            this.consumed__109 = consumed__124;
            this.stateAfter__110 = stateAfter__125;
        }
        public string AdjustedString
        {
            get
            {
                return this.adjustedString__108;
            }
        }
        public int Consumed
        {
            get
            {
                return this.consumed__109;
            }
        }
        public AutoescState<CONTEXT__11> StateAfter
        {
            get
            {
                return this.stateAfter__110;
            }
        }
    }
}
