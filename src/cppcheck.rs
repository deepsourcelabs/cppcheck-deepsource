use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    #[serde(rename = "@file")]
    pub file: String,
    #[serde(rename = "@line")]
    pub line: u32,
    #[serde(rename = "@column")]
    pub column: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Error {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@severity")]
    pub severity: String,
    #[serde(rename = "@msg")]
    pub msg: String,
    #[serde(rename = "@verbose")]
    pub verbose: String,
    #[serde(rename = "@file0")]
    pub file0: Option<String>,
    #[serde(rename = "@cwe")]
    pub cwe: Option<String>,
    pub location: Option<Vec<Location>>,
    pub symbol: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Errors {
    #[serde(default)]
    pub error: Vec<Error>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Results {
    pub errors: Errors,
}

#[test]
fn t_xml() {
    let src = r##"
    <?xml version="1.0" encoding="UTF-8"?>
    <results version="2">
        <cppcheck version="2.10.3"/>
        <errors>
            <error id="missingReturn" severity="error" msg="Found an exit path from function with non-void return type that has missing return statement" verbose="Found an exit path from function with non-void return type that has missing return statement" cwe="758" file0="test/checks/a1002.c">
                <location file="test/checks/a1002.c" line="4" column="5"/>
            </error>
            <error id="misra-c2012-2.3" severity="style" msg="misra violation (use --rule-texts=&lt;file&gt; to get proper output)" verbose="misra violation (use --rule-texts=&lt;file&gt; to get proper output)">
                <location file="test/checks/use-using.cpp" line="196" column="45"/>
            </error>
            <error id="misra-c2012-2.3" severity="style" msg="misra violation (use --rule-texts=&lt;file&gt; to get proper output)" verbose="misra violation (use --rule-texts=&lt;file&gt; to get proper output)">
                <location file="test/checks/use-using.cpp" line="198" column="90"/>
            </error>
            <error id="misra-c2012-2.3" severity="style" msg="misra violation (use --rule-texts=&lt;file&gt; to get proper output)" verbose="misra violation (use --rule-texts=&lt;file&gt; to get proper output)">
                <location file="test/checks/use-using.cpp" line="200" column="111"/>
            </error>
            <error id="misra-c2012-2.5" severity="style" msg="misra violation (use --rule-texts=&lt;file&gt; to get proper output)" verbose="misra violation (use --rule-texts=&lt;file&gt; to get proper output)">
                <location file="test/checks/test_line_len_limit.c" line="1" column="0"/>
            </error>
            <error id="misra-c2012-5.8" severity="style" msg="misra violation (use --rule-texts=&lt;file&gt; to get proper output)" verbose="misra violation (use --rule-texts=&lt;file&gt; to get proper output)">
                <location file="test/checks/flp30-c.c" line="9" column="9"/>
            </error>
            <error id="misra-c2012-5.8" severity="style" msg="misra violation (use --rule-texts=&lt;file&gt; to get proper output)" verbose="misra violation (use --rule-texts=&lt;file&gt; to get proper output)">
                <location file="test/checks/msc32-c.c" line="26" column="7"/>
            </error>
        </errors>
    </results>
    "##;
    let _value: Results = quick_xml::de::from_str(src).unwrap();
}

pub(crate) fn mapping(s: &str) -> Option<String> {
    Some(
        match s {
            // compiler only issue
            "misra-c2012-1.1" => "CXX-W3001",
            "misra-c2012-1.2" => "CXX-W3002",
            // implmented in CppCheck
            "misra-c2012-1.3" => "CXX-W3003",
            "misra-c2012-1.4" => "CXX-W3004",
            "misra-c2012-2.1" => "CXX-W3005",
            "misra-c2012-2.2" => "CXX-W3006",
            "misra-c2012-2.3" => "CXX-W3007",
            "misra-c2012-2.4" => "CXX-W3008",
            "misra-c2012-2.5" => "CXX-W3009",
            "misra-c2012-2.6" => "CXX-W3010",
            "misra-c2012-2.7" => "CXX-W3011",
            "misra-c2012-3.1" => "CXX-W3012",
            "misra-c2012-3.2" => "CXX-W3013",
            "misra-c2012-4.1" => "CXX-W3014",
            "misra-c2012-4.2" => "CXX-W3015",
            "misra-c2012-5.1" => "CXX-W3016",
            "misra-c2012-5.2" => "CXX-W3017",
            "misra-c2012-5.3" => "CXX-W3018",
            "misra-c2012-5.4" => "CXX-W3019",
            "misra-c2012-5.5" => "CXX-W3020",
            "misra-c2012-5.6" => "CXX-W3021",
            "misra-c2012-5.7" => "CXX-W3022",
            "misra-c2012-5.8" => "CXX-W3023",
            "misra-c2012-5.9" => "CXX-W3024",
            "misra-c2012-6.1" => "CXX-W3025",
            "misra-c2012-6.2" => "CXX-W3026",
            "misra-c2012-7.1" => "CXX-W3027",
            "misra-c2012-7.2" => "CXX-W3028",
            "misra-c2012-7.3" => "CXX-W3029",
            "misra-c2012-7.4" => "CXX-W3030",
            "misra-c2012-8.1" => "CXX-W3031",
            "misra-c2012-8.2" => "CXX-W3032",
            "misra-c2012-8.3" => "CXX-W3033",
            "misra-c2012-8.4" => "CXX-W3034",
            "misra-c2012-8.5" => "CXX-W3035",
            "misra-c2012-8.6" => "CXX-W3036",
            "misra-c2012-8.7" => "CXX-W3037",
            "misra-c2012-8.8" => "CXX-W3038",
            "misra-c2012-8.9" => "CXX-W3039",
            "misra-c2012-8.10" => "CXX-W3040",
            "misra-c2012-8.11" => "CXX-W3041",
            "misra-c2012-8.12" => "CXX-W3042",
            "misra-c2012-8.13" => "CXX-W3043",
            "misra-c2012-8.14" => "CXX-W3044",
            "misra-c2012-9.1" => "CXX-W3045",
            "misra-c2012-9.2" => "CXX-W3046",
            "misra-c2012-9.3" => "CXX-W3047",
            "misra-c2012-9.4" => "CXX-W3048",
            "misra-c2012-9.5" => "CXX-W3049",
            "misra-c2012-10.1" => "CXX-W3050",
            "misra-c2012-10.2" => "CXX-W3051",
            "misra-c2012-10.3" => "CXX-W3052",
            "misra-c2012-10.4" => "CXX-W3053",
            "misra-c2012-10.5" => "CXX-W3054",
            "misra-c2012-10.6" => "CXX-W3055",
            "misra-c2012-10.7" => "CXX-W3056",
            "misra-c2012-10.8" => "CXX-W3057",
            "misra-c2012-11.1" => "CXX-W3058",
            "misra-c2012-11.2" => "CXX-W3059",
            "misra-c2012-11.3" => "CXX-W3060",
            "misra-c2012-11.4" => "CXX-W3061",
            "misra-c2012-11.5" => "CXX-W3062",
            "misra-c2012-11.6" => "CXX-W3063",
            "misra-c2012-11.7" => "CXX-W3064",
            "misra-c2012-11.8" => "CXX-W3065",
            "misra-c2012-11.9" => "CXX-W3066",
            "misra-c2012-12.1" => "CXX-W3067",
            "misra-c2012-12.2" => "CXX-W3068",
            "misra-c2012-12.3" => "CXX-W3069",
            "misra-c2012-12.4" => "CXX-W3070",
            "misra-c2012-13.1" => "CXX-W3071",
            "misra-c2012-13.2" => "CXX-W3072",
            "misra-c2012-13.3" => "CXX-W3073",
            "misra-c2012-13.4" => "CXX-W3074",
            "misra-c2012-13.5" => "CXX-W3075",
            "misra-c2012-13.6" => "CXX-W3076",
            "misra-c2012-14.1" => "CXX-W3077",
            "misra-c2012-14.2" => "CXX-W3078",
            "misra-c2012-14.3" => "CXX-W3079",
            "misra-c2012-14.4" => "CXX-W3080",
            "misra-c2012-15.1" => "CXX-W3081",
            "misra-c2012-15.2" => "CXX-W3082",
            "misra-c2012-15.3" => "CXX-W3083",
            "misra-c2012-15.4" => "CXX-W3084",
            "misra-c2012-15.5" => "CXX-W3085",
            "misra-c2012-15.6" => "CXX-W3086",
            "misra-c2012-15.7" => "CXX-W3087",
            "misra-c2012-16.1" => "CXX-W3088",
            "misra-c2012-16.2" => "CXX-W3089",
            "misra-c2012-16.3" => "CXX-W3090",
            "misra-c2012-16.4" => "CXX-W3091",
            "misra-c2012-16.5" => "CXX-W3092",
            "misra-c2012-16.6" => "CXX-W3093",
            "misra-c2012-16.7" => "CXX-W3094",
            "misra-c2012-17.1" => "CXX-W3095",
            "misra-c2012-17.2" => "CXX-W3096",
            // compiler only issue
            "misra-c2012-17.3" => "CXX-W3097",
            // -------------------
            "misra-c2012-17.4" => "CXX-W3098",
            "misra-c2012-17.5" => "CXX-W3099",
            "misra-c2012-17.6" => "CXX-W3100",
            "misra-c2012-17.7" => "CXX-W3101",
            "misra-c2012-17.8" => "CXX-W3102",
            "misra-c2012-18.1" => "CXX-W3103",
            "misra-c2012-18.2" => "CXX-W3104",
            "misra-c2012-18.3" => "CXX-W3105",
            "misra-c2012-18.4" => "CXX-W3106",
            "misra-c2012-18.5" => "CXX-W3107",
            "misra-c2012-18.6" => "CXX-W3108",
            "misra-c2012-18.7" => "CXX-W3109",
            "misra-c2012-18.8" => "CXX-W3110",
            "misra-c2012-19.1" => "CXX-W3111",
            "misra-c2012-19.2" => "CXX-W3112",
            "misra-c2012-20.1" => "CXX-W3113",
            "misra-c2012-20.2" => "CXX-W3114",
            "misra-c2012-20.3" => "CXX-W3115",
            "misra-c2012-20.4" => "CXX-W3116",
            "misra-c2012-20.5" => "CXX-W3117",
            "misra-c2012-20.6" => "CXX-W3118",
            "misra-c2012-20.7" => "CXX-W3119",
            "misra-c2012-20.8" => "CXX-W3120",
            "misra-c2012-20.9" => "CXX-W3121",
            "misra-c2012-20.10" => "CXX-W3122",
            "misra-c2012-20.11" => "CXX-W3123",
            "misra-c2012-20.12" => "CXX-W3124",
            "misra-c2012-20.13" => "CXX-W3125",
            "misra-c2012-20.14" => "CXX-W3126",
            "misra-c2012-21.1" => "CXX-W3127",
            "misra-c2012-21.2" => "CXX-W3128",
            "misra-c2012-21.3" => "CXX-W3129",
            "misra-c2012-21.4" => "CXX-W3130",
            "misra-c2012-21.5" => "CXX-W3131",
            "misra-c2012-21.6" => "CXX-W3132",
            "misra-c2012-21.7" => "CXX-W3133",
            "misra-c2012-21.8" => "CXX-W3134",
            "misra-c2012-21.9" => "CXX-W3135",
            "misra-c2012-21.10" => "CXX-W3136",
            "misra-c2012-21.11" => "CXX-W3137",
            "misra-c2012-21.12" => "CXX-W3138",
            "misra-c2012-21.13" => "CXX-W3139",
            "misra-c2012-21.14" => "CXX-W3140",
            "misra-c2012-21.15" => "CXX-W3141",
            "misra-c2012-21.16" => "CXX-W3142",
            "misra-c2012-21.17" => "CXX-W3143",
            "misra-c2012-21.18" => "CXX-W3144",
            "misra-c2012-21.19" => "CXX-W3145",
            "misra-c2012-21.20" => "CXX-W3146",
            "misra-c2012-21.21" => "CXX-W3147",
            "misra-c2012-22.1" => "CXX-W3148",
            "misra-c2012-22.2" => "CXX-W3149",
            "misra-c2012-22.3" => "CXX-W3150",
            "misra-c2012-22.4" => "CXX-W3151",
            "misra-c2012-22.5" => "CXX-W3152",
            "misra-c2012-22.6" => "CXX-W3153",
            "misra-c2012-22.7" => "CXX-W3154",
            "misra-c2012-22.8" => "CXX-W3155",
            "misra-c2012-22.9" => "CXX-W3156",
            "misra-c2012-22.10" => "CXX-W3157",
            // all other cpp check issues
            "purgedConfiguration" => "CXX-W3500",
            "toomanyconfigs" => "CXX-W3501",
            "AssignmentAddressToInteger" => "CXX-W3502",
            "AssignmentIntegerToAddress" => "CXX-W3503",
            "CastIntegerToAddressAtReturn" => "CXX-W3504",
            "CastAddressToIntegerAtReturn" => "CXX-W3505",
            "assertWithSideEffect" => "CXX-W3506",
            "assignmentInAssert" => "CXX-W3507",
            "autoVariables" => "CXX-W3508",
            "returnReference" => "CXX-W3509",
            "danglingReference" => "CXX-W3510",
            "returnTempReference" => "CXX-W3511",
            "danglingTempReference" => "CXX-W3512",
            "autovarInvalidDeallocation" => "CXX-W3513",
            "uselessAssignmentArg" => "CXX-W3514",
            "uselessAssignmentPtrArg" => "CXX-W3515",
            "returnDanglingLifetime" => "CXX-W3516",
            "invalidLifetime" => "CXX-W3517",
            "danglingLifetime" => "CXX-W3518",
            "danglingTemporaryLifetime" => "CXX-W3519",
            "assignBoolToPointer" => "CXX-W3520",
            "assignBoolToFloat" => "CXX-W3521",
            "comparisonOfFuncReturningBoolError" => "CXX-W3522",
            "comparisonOfTwoFuncsReturningBoolError" => "CXX-W3523",
            "comparisonOfBoolWithBoolError" => "CXX-W3524",
            "incrementboolean" => "CXX-W3525",
            "bitwiseOnBoolean" => "CXX-W3526",
            "compareBoolExpressionWithInt" => "CXX-W3527",
            "pointerArithBool" => "CXX-W3528",
            "comparisonOfBoolWithInvalidComparator" => "CXX-W3529",
            "returnNonBoolInBooleanFunction" => "CXX-W3530",
            "boostForeachError" => "CXX-W3531",
            "arrayIndexOutOfBounds" => "CXX-W3532",
            "arrayIndexOutOfBoundsCond" => "CXX-W3533",
            "pointerOutOfBounds" => "CXX-W3534",
            "negativeIndex" => "CXX-W3535",
            "arrayIndexThenCheck" => "CXX-W3536",
            "bufferAccessOutOfBounds" => "CXX-W3537",
            "objectIndex" => "CXX-W3538",
            "argumentSize" => "CXX-W3539",
            "negativeMemoryAllocationSize" => "CXX-W3540",
            "negativeArraySize" => "CXX-W3541",
            "invalidFunctionArg" => "CXX-W3542",
            "invalidFunctionArgBool" => "CXX-W3543",
            "invalidFunctionArgStr" => "CXX-W3544",
            "ignoredReturnValue" => "CXX-W3545",
            "wrongmathcall" => "CXX-W3546",
            "unpreciseMathCall" => "CXX-W3547",
            "memsetZeroBytes" => "CXX-W3548",
            "memsetFloat" => "CXX-W3549",
            "memsetValueOutOfRange" => "CXX-W3550",
            "missingReturn" => "CXX-W3551",
            "returnStdMoveLocal" => "CXX-W3552",
            "useStandardLibrary" => "CXX-W3553",
            "noConstructor" => "CXX-W3554",
            "noExplicitConstructor" => "CXX-W3555",
            "copyCtorPointerCopying" => "CXX-W3556",
            "noCopyConstructor" => "CXX-W3557",
            "noOperatorEq" => "CXX-W3558",
            "noDestructor" => "CXX-W3559",
            "uninitMemberVar" => "CXX-W3560",
            "uninitMemberVarPrivate" => "CXX-W3561",
            "uninitDerivedMemberVar" => "CXX-W3562",
            "uninitDerivedMemberVarPrivate" => "CXX-W3563",
            "missingMemberCopy" => "CXX-W3564",
            "operatorEqVarError" => "CXX-W3565",
            "unusedPrivateFunction" => "CXX-W3566",
            "memsetClass" => "CXX-W3567",
            "memsetClassReference" => "CXX-W3568",
            "memsetClassFloat" => "CXX-W3569",
            "mallocOnClassWarning" => "CXX-W3570",
            "mallocOnClassError" => "CXX-W3571",
            "virtualDestructor" => "CXX-W3572",
            "thisSubtraction" => "CXX-W3573",
            "operatorEqRetRefThis" => "CXX-W3574",
            "operatorEqMissingReturnStatement" => "CXX-W3575",
            "operatorEqShouldBeLeftUnimplemented" => "CXX-W3576",
            "operatorEqToSelf" => "CXX-W3577",
            "functionConst" => "CXX-W3578",
            "functionStatic" => "CXX-W3579",
            "initializerList" => "CXX-W3580",
            "useInitializationList" => "CXX-W3581",
            "selfInitialization" => "CXX-W3582",
            "duplInheritedMember" => "CXX-W3583",
            "copyCtorAndEqOperator" => "CXX-W3584",
            "pureVirtualCall" => "CXX-W3585",
            "virtualCallInConstructor" => "CXX-W3586",
            "missingOverride" => "CXX-W3587",
            "thisUseAfterFree" => "CXX-W3588",
            "unsafeClassRefMember" => "CXX-W3589",
            "assignIfError" => "CXX-W3590",
            "badBitmaskCheck" => "CXX-W3591",
            "comparisonError" => "CXX-W3592",
            "duplicateCondition" => "CXX-W3593",
            "multiCondition" => "CXX-W3594",
            "mismatchingBitAnd" => "CXX-W3595",
            "oppositeInnerCondition" => "CXX-W3596",
            "identicalInnerCondition" => "CXX-W3597",
            "identicalConditionAfterEarlyExit" => "CXX-W3598",
            "incorrectLogicOperator" => "CXX-W3599",
            "redundantCondition" => "CXX-W3600",
            "moduloAlwaysTrueFalse" => "CXX-W3601",
            "clarifyCondition" => "CXX-W3602",
            "knownConditionTrueFalse" => "CXX-W3603",
            "invalidTestForOverflow" => "CXX-W3604",
            "pointerAdditionResultNotNull" => "CXX-W3605",
            "duplicateConditionalAssign" => "CXX-W3606",
            "assignmentInCondition" => "CXX-W3607",
            "compareValueOutOfTypeRangeError" => "CXX-W3608",
            "exceptThrowInDestructor" => "CXX-W3609",
            "exceptDeallocThrow" => "CXX-W3610",
            "exceptRethrowCopy" => "CXX-W3611",
            "catchExceptionByValue" => "CXX-W3612",
            "throwInNoexceptFunction" => "CXX-W3613",
            "unhandledExceptionSpecification" => "CXX-W3614",
            "rethrowNoCurrentException" => "CXX-W3615",
            "coutCerrMisusage" => "CXX-W3616",
            "fflushOnInputStream" => "CXX-W3617",
            "IOWithoutPositioning" => "CXX-W3618",
            "readWriteOnlyFile" => "CXX-W3619",
            "writeReadOnlyFile" => "CXX-W3620",
            "useClosedFile" => "CXX-W3621",
            "seekOnAppendedFile" => "CXX-W3622",
            "incompatibleFileOpen" => "CXX-W3623",
            "invalidscanf" => "CXX-W3624",
            "wrongPrintfScanfArgNum" => "CXX-W3625",
            "invalidScanfArgType_s" => "CXX-W3626",
            "invalidScanfArgType_int" => "CXX-W3627",
            "invalidScanfArgType_float" => "CXX-W3628",
            "invalidPrintfArgType_s" => "CXX-W3629",
            "invalidPrintfArgType_n" => "CXX-W3630",
            "invalidPrintfArgType_p" => "CXX-W3631",
            "invalidPrintfArgType_uint" => "CXX-W3632",
            "invalidPrintfArgType_sint" => "CXX-W3633",
            "invalidPrintfArgType_float" => "CXX-W3634",
            "invalidLengthModifierError" => "CXX-W3635",
            "invalidScanfFormatWidth" => "CXX-W3636",
            "invalidScanfFormatWidth_smaller" => "CXX-W3637",
            "wrongPrintfScanfParameterPositionError" => "CXX-W3638",
            "deallocret" => "CXX-W3639",
            "doubleFree" => "CXX-W3640",
            "leakNoVarFunctionCall" => "CXX-W3641",
            "leakReturnValNotUsed" => "CXX-W3642",
            "leakUnsafeArgAlloc" => "CXX-W3643",
            "publicAllocationError" => "CXX-W3644",
            "unsafeClassCanLeak" => "CXX-W3645",
            "memleak" => "CXX-W3646",
            "resourceLeak" => "CXX-W3647",
            "deallocuse" => "CXX-W3648",
            "mismatchAllocDealloc" => "CXX-W3649",
            "memleakOnRealloc" => "CXX-W3650",
            "nullPointer" => "CXX-W3651",
            "nullPointerDefaultArg" => "CXX-W3652",
            "nullPointerRedundantCheck" => "CXX-W3653",
            "nullPointerArithmetic" => "CXX-W3654",
            "nullPointerArithmeticRedundantCheck" => "CXX-W3655",
            "zerodiv" => "CXX-W3656",
            "zerodivcond" => "CXX-W3657",
            "unusedScopedObject" => "CXX-W3658",
            "invalidPointerCast" => "CXX-W3659",
            "shiftNegativeLHS" => "CXX-W3660",
            "shiftNegative" => "CXX-W3661",
            "raceAfterInterlockedDecrement" => "CXX-W3662",
            "invalidFree" => "CXX-W3663",
            "overlappingWriteUnion" => "CXX-W3664",
            "overlappingWriteFunction" => "CXX-W3665",
            "redundantCopyLocalConst" => "CXX-W3666",
            "redundantCopy" => "CXX-W3667",
            "comparisonFunctionIsAlwaysTrueOrFalse" => "CXX-W3668",
            "checkCastIntToCharAndBack" => "CXX-W3669",
            "cstyleCast" => "CXX-W3670",
            "passedByValue" => "CXX-W3671",
            "constParameter" => "CXX-W3672",
            "constVariable" => "CXX-W3673",
            "constParameterCallback" => "CXX-W3674",
            "constStatement" => "CXX-W3675",
            "signedCharArrayIndex" => "CXX-W3676",
            "unknownSignCharArrayIndex" => "CXX-W3677",
            "charBitOp" => "CXX-W3678",
            "variableScope" => "CXX-W3679",
            "redundantAssignInSwitch" => "CXX-W3680",
            "suspiciousCase" => "CXX-W3681",
            "selfAssignment" => "CXX-W3682",
            "clarifyCalculation" => "CXX-W3683",
            "clarifyStatement" => "CXX-W3684",
            "duplicateBranch" => "CXX-W3685",
            "duplicateAssignExpression" => "CXX-W3686",
            "oppositeExpression" => "CXX-W3687",
            "duplicateExpression" => "CXX-W3688",
            "duplicateValueTernary" => "CXX-W3689",
            "duplicateExpressionTernary" => "CXX-W3690",
            "duplicateBreak" => "CXX-W3691",
            "unreachableCode" => "CXX-W3692",
            "unsignedLessThanZero" => "CXX-W3693",
            "unsignedPositive" => "CXX-W3694",
            "pointerLessThanZero" => "CXX-W3695",
            "pointerPositive" => "CXX-W3696",
            "suspiciousSemicolon" => "CXX-W3697",
            "incompleteArrayFill" => "CXX-W3698",
            "varFuncNullUB" => "CXX-W3699",
            "nanInArithmeticExpression" => "CXX-W3700",
            "commaSeparatedReturn" => "CXX-W3701",
            "redundantPointerOp" => "CXX-W3702",
            "unusedLabel" => "CXX-W3703",
            "unusedLabelConfiguration" => "CXX-W3704",
            "unusedLabelSwitch" => "CXX-W3705",
            "unusedLabelSwitchConfiguration" => "CXX-W3706",
            "unknownEvaluationOrder" => "CXX-W3707",
            "accessMoved" => "CXX-W3708",
            "accessForwarded" => "CXX-W3709",
            "funcArgNamesDifferent" => "CXX-W3710",
            "redundantBitwiseOperationInSwitch" => "CXX-W3711",
            "shadowVariable" => "CXX-W3712",
            "shadowFunction" => "CXX-W3713",
            "shadowArgument" => "CXX-W3714",
            "knownArgument" => "CXX-W3715",
            "knownArgumentHiddenVariableExpression" => "CXX-W3716",
            "comparePointers" => "CXX-W3717",
            "redundantAssignment" => "CXX-W3718",
            "redundantInitialization" => "CXX-W3719",
            "funcArgOrderDifferent" => "CXX-W3720",
            "moduloofone" => "CXX-W3721",
            "containerOutOfBounds" => "CXX-W3722",
            "invalidIterator1" => "CXX-W3723",
            "iterators1" => "CXX-W3724",
            "iterators2" => "CXX-W3725",
            "iterators3" => "CXX-W3726",
            "invalidContainerLoop" => "CXX-W3727",
            "invalidContainer" => "CXX-W3728",
            "mismatchingContainerIterator" => "CXX-W3729",
            "mismatchingContainers" => "CXX-W3730",
            "mismatchingContainerExpression" => "CXX-W3731",
            "sameIteratorExpression" => "CXX-W3732",
            "eraseDereference" => "CXX-W3733",
            "stlOutOfBounds" => "CXX-W3734",
            "negativeContainerIndex" => "CXX-W3735",
            "stlBoundaries" => "CXX-W3736",
            "stlIfFind" => "CXX-W3737",
            "stlIfStrFind" => "CXX-W3738",
            "stlFindInsert" => "CXX-W3739",
            "stlcstr" => "CXX-W3740",
            "stlcstrReturn" => "CXX-W3741",
            "stlcstrParam" => "CXX-W3742",
            "stlcstrthrow" => "CXX-W3743",
            "stlSize" => "CXX-W3744",
            "StlMissingComparison" => "CXX-W3745",
            "redundantIfRemove" => "CXX-W3746",
            "uselessCallsCompare" => "CXX-W3747",
            "uselessCallsSwap" => "CXX-W3748",
            "uselessCallsSubstr" => "CXX-W3749",
            "uselessCallsEmpty" => "CXX-W3750",
            "uselessCallsRemove" => "CXX-W3751",
            "derefInvalidIterator" => "CXX-W3752",
            "useStlAlgorithm" => "CXX-W3753",
            "knownEmptyContainer" => "CXX-W3754",
            "globalLockGuard" => "CXX-W3755",
            "localMutex" => "CXX-W3756",
            "sizeofwithsilentarraypointer" => "CXX-W3757",
            "pointerSize" => "CXX-W3758",
            "sizeofDivisionMemfunc" => "CXX-W3759",
            "sizeofwithnumericparameter" => "CXX-W3760",
            "sizeofsizeof" => "CXX-W3761",
            "sizeofCalculation" => "CXX-W3762",
            "sizeofFunctionCall" => "CXX-W3763",
            "multiplySizeof" => "CXX-W3764",
            "divideSizeof" => "CXX-W3765",
            "sizeofVoid" => "CXX-W3766",
            "sizeofDereferencedVoidPointer" => "CXX-W3767",
            "arithOperationsOnVoidPointer" => "CXX-W3768",
            "stringLiteralWrite" => "CXX-W3769",
            "sprintfOverlappingData" => "CXX-W3770",
            "strPlusChar" => "CXX-W3771",
            "incorrectStringCompare" => "CXX-W3772",
            "literalWithCharPtrCompare" => "CXX-W3773",
            "charLiteralWithCharPtrCompare" => "CXX-W3774",
            "incorrectStringBooleanError" => "CXX-W3775",
            "incorrectCharBooleanError" => "CXX-W3776",
            "staticStringCompare" => "CXX-W3777",
            "stringCompare" => "CXX-W3778",
            "overlappingStrcmp" => "CXX-W3779",
            "shiftTooManyBits" => "CXX-W3780",
            "shiftTooManyBitsSigned" => "CXX-W3781",
            "integerOverflow" => "CXX-W3782",
            "signConversion" => "CXX-W3783",
            "truncLongCastAssignment" => "CXX-W3784",
            "truncLongCastReturn" => "CXX-W3785",
            "floatConversionOverflow" => "CXX-W3786",
            "uninitdata" => "CXX-W3787",
            "uninitStructMember" => "CXX-W3788",
            "unusedFunction" => "CXX-W3789",
            "unusedVariable" => "CXX-W3790",
            "unusedAllocatedMemory" => "CXX-W3791",
            "unreadVariable" => "CXX-W3792",
            "unassignedVariable" => "CXX-W3793",
            "unusedStructMember" => "CXX-W3794",
            "postfixOperator" => "CXX-W3795",
            "va_start_wrongParameter" => "CXX-W3796",
            "va_start_referencePassed" => "CXX-W3797",
            "va_end_missing" => "CXX-W3798",
            "va_list_usedBeforeStarted" => "CXX-W3799",
            "va_start_subsequentCalls" => "CXX-W3800",
            "missingInclude" => "CXX-W3801",
            "missingIncludeSystem" => "CXX-W3802",
            "ConfigurationNotChecked" => "CXX-W3803",
            "preprocessorErrorDirective" => "CXX-W3804",
            _ => return None,
        }
        .to_string(),
    )
}
