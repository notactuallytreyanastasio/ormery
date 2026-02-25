namespace Ormery.Core
{
    public interface IContextPropagator<CONTEXT__15> where CONTEXT__15: IContext
    {
        AfterPropagate<CONTEXT__15> After(AutoescState<CONTEXT__15> before__127, string ? literalPart__128);
    }
}
