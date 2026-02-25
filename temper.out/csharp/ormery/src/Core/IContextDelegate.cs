using C = Ormery.Core;
namespace Ormery.Core
{
    public interface IContextDelegate<CONTEXT__18>: IDelegate where CONTEXT__18: IContext
    {
        protected AutoescState<CONTEXT__18> State
        {
            get;
            set;
        }
        protected IContextPropagator<CONTEXT__18> ContextPropagator
        {
            get;
        }
        protected static string ProcessDefault(IContextDelegate<CONTEXT__18> this__22, string ? known__141)
        {
            AfterPropagate<CONTEXT__18> after__143 = C::CoreGlobal.PropagateOver(this__22.ContextPropagator, this__22.State, known__141);
            AutoescState<CONTEXT__18> t___479 = after__143.StateAfter;
            this__22.State = t___479;
            return after__143.AdjustedString;
        }
        string Process(string ? known__141)
        {
            return ProcessDefault(this, known__141);
        }
    }
}
