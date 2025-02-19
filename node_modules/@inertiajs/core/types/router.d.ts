import { AxiosResponse } from 'axios';
import { ActiveVisit, GlobalEvent, GlobalEventNames, GlobalEventResult, LocationVisit, Page, PageHandler, PageResolver, PreserveStateOption, RequestPayload, VisitId, VisitOptions } from './types';
export declare class Router {
    protected page: Page;
    protected resolveComponent: PageResolver;
    protected swapComponent: PageHandler;
    protected navigationType?: string;
    protected activeVisit?: ActiveVisit;
    protected visitId: VisitId;
    init({ initialPage, resolveComponent, swapComponent, }: {
        initialPage: Page;
        resolveComponent: PageResolver;
        swapComponent: PageHandler;
    }): void;
    protected setNavigationType(): void;
    protected clearRememberedStateOnReload(): void;
    protected handleInitialPageVisit(page: Page): void;
    protected setupEventListeners(): void;
    protected scrollRegions(): NodeListOf<Element>;
    protected handleScrollEvent(event: Event): void;
    protected saveScrollPositions(): void;
    protected resetScrollPositions(): void;
    protected restoreScrollPositions(): void;
    protected isBackForwardVisit(): boolean;
    protected handleBackForwardVisit(page: Page): void;
    protected locationVisit(url: URL, preserveScroll: LocationVisit['preserveScroll']): boolean | void;
    protected isLocationVisit(): boolean;
    protected handleLocationVisit(page: Page): void;
    protected isLocationVisitResponse(response: AxiosResponse): boolean;
    protected isInertiaResponse(response: AxiosResponse): boolean;
    protected createVisitId(): VisitId;
    protected cancelVisit(activeVisit: ActiveVisit, { cancelled, interrupted }: {
        cancelled?: boolean;
        interrupted?: boolean;
    }): void;
    protected finishVisit(visit: ActiveVisit): void;
    protected resolvePreserveOption(value: PreserveStateOption, page: Page): boolean;
    cancel(): void;
    visit(href: string | URL, { method, data, replace, preserveScroll, preserveState, only, except, headers, errorBag, forceFormData, onCancelToken, onBefore, onStart, onProgress, onFinish, onCancel, onSuccess, onError, queryStringArrayFormat, }?: VisitOptions): void;
    protected setPage(page: Page, { visitId, replace, preserveScroll, preserveState, }?: {
        visitId?: VisitId;
        replace?: boolean;
        preserveScroll?: PreserveStateOption;
        preserveState?: PreserveStateOption;
    }): Promise<void>;
    protected pushState(page: Page): void;
    protected replaceState(page: Page): void;
    protected handlePopstateEvent(event: PopStateEvent): void;
    get(url: URL | string, data?: RequestPayload, options?: Omit<VisitOptions, 'method' | 'data'>): void;
    reload(options?: Omit<VisitOptions, 'preserveScroll' | 'preserveState'>): void;
    replace(url: URL | string, options?: Omit<VisitOptions, 'replace'>): void;
    post(url: URL | string, data?: RequestPayload, options?: Omit<VisitOptions, 'method' | 'data'>): void;
    put(url: URL | string, data?: RequestPayload, options?: Omit<VisitOptions, 'method' | 'data'>): void;
    patch(url: URL | string, data?: RequestPayload, options?: Omit<VisitOptions, 'method' | 'data'>): void;
    delete(url: URL | string, options?: Omit<VisitOptions, 'method'>): void;
    remember(data: unknown, key?: string): void;
    restore(key?: string): unknown;
    on<TEventName extends GlobalEventNames>(type: TEventName, callback: (event: GlobalEvent<TEventName>) => GlobalEventResult<TEventName>): VoidFunction;
}
