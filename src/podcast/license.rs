use serde::{Deserialize, Deserializer};
use std::str::FromStr;
use strum_macros::{Display, EnumString};

// TODO: improve variant names.
#[derive(Debug, PartialEq, Eq, EnumString, Display)]
pub enum LicenseType {
    #[strum(serialize = "0bsd")]
    BSDZeroClauseLicense,
    #[strum(serialize = "aal")]
    AttributionAssuranceLicense,
    #[strum(serialize = "abstyles")]
    AbstylesLicense,
    #[strum(serialize = "adobe-2006")]
    AdobeSystemsIncorporatedSourceCodeLicenseAgreement,
    #[strum(serialize = "adobe-glyph")]
    AdobeGlyphListLicense,
    #[strum(serialize = "adsl")]
    AmazonDigitalServicesLicense,
    #[strum(serialize = "afl-1.1")]
    AcademicFreeLicensev1_1,
    #[strum(serialize = "afl-1.2")]
    AcademicFreeLicensev1_2,
    #[strum(serialize = "afl-2.0")]
    AcademicFreeLicensev2_0,
    #[strum(serialize = "afl-2.1")]
    AcademicFreeLicensev2_1,
    #[strum(serialize = "afl-3.0")]
    AcademicFreeLicensev3_0,
    #[strum(serialize = "afmparse")]
    AfmparseLicense,
    #[strum(serialize = "agpl-1.0-only")]
    AfferoGeneralPublicLicensev1_0only,
    #[strum(serialize = "agpl-1.0-or-later")]
    AfferoGeneralPublicLicensev1_0orlater,
    #[strum(serialize = "agpl-3.0-only")]
    GNUAfferoGeneralPublicLicensev3_0only,
    #[strum(serialize = "agpl-3.0-or-later")]
    GNUAfferoGeneralPublicLicensev3_0orlater,
    #[strum(serialize = "aladdin")]
    AladdinFreePublicLicense,
    #[strum(serialize = "amdplpa")]
    AMDPlpaMapCLicense,
    #[strum(serialize = "aml")]
    AppleMITLicense,
    #[strum(serialize = "ampas")]
    AcademyofMotionPictureArtsandSciencesBSD,
    #[strum(serialize = "antlr-pd")]
    ANTLRSoftwareRightsNotice,
    #[strum(serialize = "antlr-pd-fallback")]
    ANTLRSoftwareRightsNoticewithlicensefallback,
    #[strum(serialize = "apache-1.0")]
    ApacheLicense1_0,
    #[strum(serialize = "apache-1.1")]
    ApacheLicense1_1,
    #[strum(serialize = "apache-2.0")]
    ApacheLicense2_0,
    #[strum(serialize = "apafml")]
    AdobePostscriptAFMLicense,
    #[strum(serialize = "apl-1.0")]
    AdaptivePublicLicense1_0,
    #[strum(serialize = "app-s2p")]
    AppS2pLicense,
    #[strum(serialize = "apsl-1.0")]
    ApplePublicSourceLicense1_0,
    #[strum(serialize = "apsl-1.1")]
    ApplePublicSourceLicense1_1,
    #[strum(serialize = "apsl-1.2")]
    ApplePublicSourceLicense1_2,
    #[strum(serialize = "apsl-2.0")]
    ApplePublicSourceLicense2_0,
    #[strum(serialize = "arphic-1999")]
    ArphicPublicLicense,
    #[strum(serialize = "artistic-1.0")]
    ArtisticLicense1_0,
    #[strum(serialize = "artistic-1.0-cl8")]
    ArtisticLicense1_0wClause8,
    #[strum(serialize = "artistic-1.0-perl")]
    ArtisticLicense1_0Perl,
    #[strum(serialize = "artistic-2.0")]
    ArtisticLicense2_0,
    #[strum(serialize = "baekmuk")]
    BaekmukLicense,
    #[strum(serialize = "bahyph")]
    BahyphLicense,
    #[strum(serialize = "barr")]
    BarrLicense,
    #[strum(serialize = "beerware")]
    BeerwareLicense,
    #[strum(serialize = "bitstream-vera")]
    BitstreamVeraFontLicense,
    #[strum(serialize = "bittorrent-1.0")]
    BitTorrentOpenSourceLicensev1_0,
    #[strum(serialize = "bittorrent-1.1")]
    BitTorrentOpenSourceLicensev1_1,
    #[strum(serialize = "blessing")]
    SQLiteBlessing,
    #[strum(serialize = "blueoak-1.0.0")]
    BlueOakModelLicense1_0_0,
    #[strum(serialize = "borceux")]
    Borceuxlicense,
    #[strum(serialize = "bsd-1-clause")]
    BSD1ClauseLicense,
    #[strum(serialize = "bsd-2-clause")]
    BSD2ClauseSimplifiedLicense,
    #[strum(serialize = "bsd-2-clause-patent")]
    BSD2ClausePlusPatentLicense,
    #[strum(serialize = "bsd-2-clause-views")]
    BSD2Clausewithviewssentence,
    #[strum(serialize = "bsd-3-clause")]
    BSD3ClauseNewOrRevisedLicense,
    #[strum(serialize = "bsd-3-clause-attribution")]
    BSDwithattribution,
    #[strum(serialize = "bsd-3-clause-clear")]
    BSD3ClauseClearLicense,
    #[strum(serialize = "bsd-3-clause-lbnl")]
    LawrenceBerkeleyNationalLabsBSDvariantlicense,
    #[strum(serialize = "bsd-3-clause-modification")]
    BSD3ClauseModification,
    #[strum(serialize = "bsd-3-clause-no-military-license")]
    BSD3ClauseNoMilitaryLicense,
    #[strum(serialize = "bsd-3-clause-no-nuclear-license")]
    BSD3ClauseNoNuclearLicense,
    #[strum(serialize = "bsd-3-clause-no-nuclear-license-2014")]
    BSD3ClauseNoNuclearLicense2014,
    #[strum(serialize = "bsd-3-clause-no-nuclear-warranty")]
    BSD3ClauseNoNuclearWarranty,
    #[strum(serialize = "bsd-3-clause-open-mpi")]
    BSD3ClauseOpenMPIvariant,
    #[strum(serialize = "bsd-4-clause")]
    BSD4ClauseOriginalOrOldLicense,
    #[strum(serialize = "bsd-4-clause-shortened")]
    BSD4ClauseShortened,
    #[strum(serialize = "bsd-4-clause-uc")]
    BSD4ClauseUniversityofCaliforniaSpecific,
    #[strum(serialize = "bsd-protection")]
    BSDProtectionLicense,
    #[strum(serialize = "bsd-source-code")]
    BSDSourceCodeAttribution,
    #[strum(serialize = "bsl-1.0")]
    BoostSoftwareLicense1_0,
    #[strum(serialize = "busl-1.1")]
    BusinessSourceLicense1_1,
    #[strum(serialize = "bzip2-1.0.6")]
    Bzip2AndLibbzip2Licensev1_0_6,
    #[strum(serialize = "c-uda-1.0")]
    ComputationalUseofDataAgreementv1_0,
    #[strum(serialize = "cal-1.0")]
    CryptographicAutonomyLicense1_0,
    #[strum(serialize = "cal-1.0-combined-work-exception")]
    CryptographicAutonomyLicense1_0CombinedWorkException,
    #[strum(serialize = "caldera")]
    CalderaLicense,
    #[strum(serialize = "catosl-1.1")]
    ComputerAssociatesTrustedOpenSourceLicense1_1,
    #[strum(serialize = "cc-by-1.0")]
    CreativeCommonsAttribution1_0Generic,
    #[strum(serialize = "cc-by-2.0")]
    CreativeCommonsAttribution2_0Generic,
    #[strum(serialize = "cc-by-2.5")]
    CreativeCommonsAttribution2_5Generic,
    #[strum(serialize = "cc-by-2.5-au")]
    CreativeCommonsAttribution2_5Australia,
    #[strum(serialize = "cc-by-3.0")]
    CreativeCommonsAttribution3_0Unported,
    #[strum(serialize = "cc-by-3.0-at")]
    CreativeCommonsAttribution3_0Austria,
    #[strum(serialize = "cc-by-3.0-de")]
    CreativeCommonsAttribution3_0Germany,
    #[strum(serialize = "cc-by-3.0-igo")]
    CreativeCommonsAttribution3_0IGO,
    #[strum(serialize = "cc-by-3.0-nl")]
    CreativeCommonsAttribution3_0Netherlands,
    #[strum(serialize = "cc-by-3.0-us")]
    CreativeCommonsAttribution3_0UnitedStates,
    #[strum(serialize = "cc-by-4.0")]
    CreativeCommonsAttribution4_0International,
    #[strum(serialize = "cc-by-nc-1.0")]
    CreativeCommonsAttributionNonCommercial1_0Generic,
    #[strum(serialize = "cc-by-nc-2.0")]
    CreativeCommonsAttributionNonCommercial2_0Generic,
    #[strum(serialize = "cc-by-nc-2.5")]
    CreativeCommonsAttributionNonCommercial2_5Generic,
    #[strum(serialize = "cc-by-nc-3.0")]
    CreativeCommonsAttributionNonCommercial3_0Unported,
    #[strum(serialize = "cc-by-nc-3.0-de")]
    CreativeCommonsAttributionNonCommercial3_0Germany,
    #[strum(serialize = "cc-by-nc-4.0")]
    CreativeCommonsAttributionNonCommercial4_0International,
    #[strum(serialize = "cc-by-nc-nd-1.0")]
    CreativeCommonsAttributionNonCommercialNoDerivatives1_0Generic,
    #[strum(serialize = "cc-by-nc-nd-2.0")]
    CreativeCommonsAttributionNonCommercialNoDerivatives2_0Generic,
    #[strum(serialize = "cc-by-nc-nd-2.5")]
    CreativeCommonsAttributionNonCommercialNoDerivatives2_5Generic,
    #[strum(serialize = "cc-by-nc-nd-3.0")]
    CreativeCommonsAttributionNonCommercialNoDerivatives3_0Unported,
    #[strum(serialize = "cc-by-nc-nd-3.0-de")]
    CreativeCommonsAttributionNonCommercialNoDerivatives3_0Germany,
    #[strum(serialize = "cc-by-nc-nd-3.0-igo")]
    CreativeCommonsAttributionNonCommercialNoDerivatives3_0IGO,
    #[strum(serialize = "cc-by-nc-nd-4.0")]
    CreativeCommonsAttributionNonCommercialNoDerivatives4_0International,
    #[strum(serialize = "cc-by-nc-sa-1.0")]
    CreativeCommonsAttributionNonCommercialShareAlike1_0Generic,
    #[strum(serialize = "cc-by-nc-sa-2.0")]
    CreativeCommonsAttributionNonCommercialShareAlike2_0Generic,
    #[strum(serialize = "cc-by-nc-sa-2.0-fr")]
    CreativeCommonsAttributionNonCommercialShareAlike2_0France,
    #[strum(serialize = "cc-by-nc-sa-2.0-uk")]
    CreativeCommonsAttributionNonCommercialShareAlike2_0EnglandandWales,
    #[strum(serialize = "cc-by-nc-sa-2.5")]
    CreativeCommonsAttributionNonCommercialShareAlike2_5Generic,
    #[strum(serialize = "cc-by-nc-sa-3.0")]
    CreativeCommonsAttributionNonCommercialShareAlike3_0Unported,
    #[strum(serialize = "cc-by-nc-sa-3.0-de")]
    CreativeCommonsAttributionNonCommercialShareAlike3_0Germany,
    #[strum(serialize = "cc-by-nc-sa-3.0-igo")]
    CreativeCommonsAttributionNonCommercialShareAlike3_0IGO,
    #[strum(serialize = "cc-by-nc-sa-4.0")]
    CreativeCommonsAttributionNonCommercialShareAlike4_0International,
    #[strum(serialize = "cc-by-nd-1.0")]
    CreativeCommonsAttributionNoDerivatives1_0Generic,
    #[strum(serialize = "cc-by-nd-2.0")]
    CreativeCommonsAttributionNoDerivatives2_0Generic,
    #[strum(serialize = "cc-by-nd-2.5")]
    CreativeCommonsAttributionNoDerivatives2_5Generic,
    #[strum(serialize = "cc-by-nd-3.0")]
    CreativeCommonsAttributionNoDerivatives3_0Unported,
    #[strum(serialize = "cc-by-nd-3.0-de")]
    CreativeCommonsAttributionNoDerivatives3_0Germany,
    #[strum(serialize = "cc-by-nd-4.0")]
    CreativeCommonsAttributionNoDerivatives4_0International,
    #[strum(serialize = "cc-by-sa-1.0")]
    CreativeCommonsAttributionShareAlike1_0Generic,
    #[strum(serialize = "cc-by-sa-2.0")]
    CreativeCommonsAttributionShareAlike2_0Generic,
    #[strum(serialize = "cc-by-sa-2.0-uk")]
    CreativeCommonsAttributionShareAlike2_0EnglandandWales,
    #[strum(serialize = "cc-by-sa-2.1-jp")]
    CreativeCommonsAttributionShareAlike2_1Japan,
    #[strum(serialize = "cc-by-sa-2.5")]
    CreativeCommonsAttributionShareAlike2_5Generic,
    #[strum(serialize = "cc-by-sa-3.0")]
    CreativeCommonsAttributionShareAlike3_0Unported,
    #[strum(serialize = "cc-by-sa-3.0-at")]
    CreativeCommonsAttributionShareAlike3_0Austria,
    #[strum(serialize = "cc-by-sa-3.0-de")]
    CreativeCommonsAttributionShareAlike3_0Germany,
    #[strum(serialize = "cc-by-sa-4.0")]
    CreativeCommonsAttributionShareAlike4_0International,
    #[strum(serialize = "cc-pddc")]
    CreativeCommonsPublicDomainDedicationandCertification,
    #[strum(serialize = "cc0-1.0")]
    CreativeCommonsZerov1_0Universal,
    #[strum(serialize = "cddl-1.0")]
    CommonDevelopmentandDistributionLicense1_0,
    #[strum(serialize = "cddl-1.1")]
    CommonDevelopmentandDistributionLicense1_1,
    #[strum(serialize = "cdl-1.0")]
    CommonDocumentationLicense1_0,
    #[strum(serialize = "cdla-permissive-1.0")]
    CommunityDataLicenseAgreementPermissive1_0,
    #[strum(serialize = "cdla-permissive-2.0")]
    CommunityDataLicenseAgreementPermissive2_0,
    #[strum(serialize = "cdla-sharing-1.0")]
    CommunityDataLicenseAgreementSharing1_0,
    #[strum(serialize = "cecill-1.0")]
    CeCILLFreeSoftwareLicenseAgreementv1_0,
    #[strum(serialize = "cecill-1.1")]
    CeCILLFreeSoftwareLicenseAgreementv1_1,
    #[strum(serialize = "cecill-2.0")]
    CeCILLFreeSoftwareLicenseAgreementv2_0,
    #[strum(serialize = "cecill-2.1")]
    CeCILLFreeSoftwareLicenseAgreementv2_1,
    #[strum(serialize = "cecill-b")]
    CeCILLBFreeSoftwareLicenseAgreement,
    #[strum(serialize = "cecill-c")]
    CeCILLCFreeSoftwareLicenseAgreement,
    #[strum(serialize = "cern-ohl-1.1")]
    CERNOpenHardwareLicencev1_1,
    #[strum(serialize = "cern-ohl-1.2")]
    CERNOpenHardwareLicencev1_2,
    #[strum(serialize = "cern-ohl-p-2.0")]
    CERNOpenHardwareLicenceVersion2Permissive,
    #[strum(serialize = "cern-ohl-s-2.0")]
    CERNOpenHardwareLicenceVersion2StronglyReciprocal,
    #[strum(serialize = "cern-ohl-w-2.0")]
    CERNOpenHardwareLicenceVersion2WeaklyReciprocal,
    #[strum(serialize = "clartistic")]
    ClarifiedArtisticLicense,
    #[strum(serialize = "cnri-jython")]
    CNRIJythonLicense,
    #[strum(serialize = "cnri-python")]
    CNRIPythonLicense,
    #[strum(serialize = "cnri-python-gpl-compatible")]
    CNRIPythonOpenSourceGPLCompatibleLicenseAgreement,
    #[strum(serialize = "coil-1.0")]
    CopyfreeOpenInnovationLicense,
    #[strum(serialize = "community-spec-1.0")]
    CommunitySpecificationLicense1_0,
    #[strum(serialize = "condor-1.1")]
    CondorPublicLicensev1_1,
    #[strum(serialize = "copyleft-next-0.3.0")]
    Copyleftnext0_3_0,
    #[strum(serialize = "copyleft-next-0.3.1")]
    Copyleftnext0_3_1,
    #[strum(serialize = "cpal-1.0")]
    CommonPublicAttributionLicense1_0,
    #[strum(serialize = "cpl-1.0")]
    CommonPublicLicense1_0,
    #[strum(serialize = "cpol-1.02")]
    CodeProjectOpenLicense1_02,
    #[strum(serialize = "crossword")]
    CrosswordLicense,
    #[strum(serialize = "crystalstacker")]
    CrystalStackerLicense,
    #[strum(serialize = "cua-opl-1.0")]
    CUAOfficePublicLicensev1_0,
    #[strum(serialize = "cube")]
    CubeLicense,
    #[strum(serialize = "curl")]
    CurlLicense,
    #[strum(serialize = "d-fsl-1.0")]
    DeutscheFreieSoftwareLizenz,
    #[strum(serialize = "diffmark")]
    DiffmarkLicense,
    #[strum(serialize = "dl-de-by-2.0")]
    DatalicenceGermanyAttributionVersion2_0,
    #[strum(serialize = "doc")]
    DOCLicense,
    #[strum(serialize = "dotseqn")]
    DotseqnLicense,
    #[strum(serialize = "drl-1.0")]
    DetectionRuleLicense1_0,
    #[strum(serialize = "dsdp")]
    DSDPLicense,
    #[strum(serialize = "dvipdfm")]
    DvipdfmLicense,
    #[strum(serialize = "ecl-1.0")]
    EducationalCommunityLicensev1_0,
    #[strum(serialize = "ecl-2.0")]
    EducationalCommunityLicensev2_0,
    #[strum(serialize = "efl-1.0")]
    EiffelForumLicensev1_0,
    #[strum(serialize = "efl-2.0")]
    EiffelForumLicensev2_0,
    #[strum(serialize = "egenix")]
    EGenixComPublicLicense1_1_0,
    #[strum(serialize = "elastic-2.0")]
    ElasticLicense2_0,
    #[strum(serialize = "entessa")]
    EntessaPublicLicensev1_0,
    #[strum(serialize = "epics")]
    EPICSOpenLicense,
    #[strum(serialize = "epl-1.0")]
    EclipsePublicLicense1_0,
    #[strum(serialize = "epl-2.0")]
    EclipsePublicLicense2_0,
    #[strum(serialize = "erlpl-1.1")]
    ErlangPublicLicensev1_1,
    #[strum(serialize = "etalab-2.0")]
    EtalabOpenLicense2_0,
    #[strum(serialize = "eudatagrid")]
    EUDataGridSoftwareLicense,
    #[strum(serialize = "eupl-1.0")]
    EuropeanUnionPublicLicense1_0,
    #[strum(serialize = "eupl-1.1")]
    EuropeanUnionPublicLicense1_1,
    #[strum(serialize = "eupl-1.2")]
    EuropeanUnionPublicLicense1_2,
    #[strum(serialize = "eurosym")]
    EurosymLicense,
    #[strum(serialize = "fair")]
    FairLicense,
    #[strum(serialize = "fdk-aac")]
    FraunhoferFDKAACCodecLibrary,
    #[strum(serialize = "frameworx-1.0")]
    FrameworxOpenLicense1_0,
    #[strum(serialize = "freebsd-doc")]
    FreeBSDDocumentationLicense,
    #[strum(serialize = "freeimage")]
    FreeImagePublicLicensev1_0,
    #[strum(serialize = "fsfap")]
    FSFAllPermissiveLicense,
    #[strum(serialize = "fsful")]
    FSFUnlimitedLicense,
    #[strum(serialize = "fsfullr")]
    FSFUnlimitedLicenseWithLicenseRetention,
    #[strum(serialize = "ftl")]
    FreetypeProjectLicense,
    #[strum(serialize = "gd")]
    GDLicense,
    #[strum(serialize = "gfdl-1.1-invariants-only")]
    GNUFreeDocumentationLicensev1_1onlyinvariants,
    #[strum(serialize = "gfdl-1.1-invariants-or-later")]
    GNUFreeDocumentationLicensev1_1orlaterinvariants,
    #[strum(serialize = "gfdl-1.1-no-invariants-only")]
    GNUFreeDocumentationLicensev1_1onlynoinvariants,
    #[strum(serialize = "gfdl-1.1-no-invariants-or-later")]
    GNUFreeDocumentationLicensev1_1orlaternoinvariants,
    #[strum(serialize = "gfdl-1.1-only")]
    GNUFreeDocumentationLicensev1_1only,
    #[strum(serialize = "gfdl-1.1-or-later")]
    GNUFreeDocumentationLicensev1_1orlater,
    #[strum(serialize = "gfdl-1.2-invariants-only")]
    GNUFreeDocumentationLicensev1_2onlyinvariants,
    #[strum(serialize = "gfdl-1.2-invariants-or-later")]
    GNUFreeDocumentationLicensev1_2orlaterinvariants,
    #[strum(serialize = "gfdl-1.2-no-invariants-only")]
    GNUFreeDocumentationLicensev1_2onlynoinvariants,
    #[strum(serialize = "gfdl-1.2-no-invariants-or-later")]
    GNUFreeDocumentationLicensev1_2orlaternoinvariants,
    #[strum(serialize = "gfdl-1.2-only")]
    GNUFreeDocumentationLicensev1_2only,
    #[strum(serialize = "gfdl-1.2-or-later")]
    GNUFreeDocumentationLicensev1_2orlater,
    #[strum(serialize = "gfdl-1.3-invariants-only")]
    GNUFreeDocumentationLicensev1_3onlyinvariants,
    #[strum(serialize = "gfdl-1.3-invariants-or-later")]
    GNUFreeDocumentationLicensev1_3orlaterinvariants,
    #[strum(serialize = "gfdl-1.3-no-invariants-only")]
    GNUFreeDocumentationLicensev1_3onlynoinvariants,
    #[strum(serialize = "gfdl-1.3-no-invariants-or-later")]
    GNUFreeDocumentationLicensev1_3orlaternoinvariants,
    #[strum(serialize = "gfdl-1.3-only")]
    GNUFreeDocumentationLicensev1_3only,
    #[strum(serialize = "gfdl-1.3-or-later")]
    GNUFreeDocumentationLicensev1_3orlater,
    #[strum(serialize = "giftware")]
    GiftwareLicense,
    #[strum(serialize = "gl2ps")]
    GL2PSLicense,
    #[strum(serialize = "glide")]
    DfxGlideLicense,
    #[strum(serialize = "glulxe")]
    GlulxeLicense,
    #[strum(serialize = "glwtpl")]
    GoodLuckWithThatPublicLicense,
    #[strum(serialize = "gnuplot")]
    GnuPlotLicense,
    #[strum(serialize = "gpl-1.0-only")]
    GNUGeneralPublicLicensev1_0only,
    #[strum(serialize = "gpl-1.0-or-later")]
    GNUGeneralPublicLicensev1_0orlater,
    #[strum(serialize = "gpl-2.0-only")]
    GNUGeneralPublicLicensev2_0only,
    #[strum(serialize = "gpl-2.0-or-later")]
    GNUGeneralPublicLicensev2_0orlater,
    #[strum(serialize = "gpl-3.0-only")]
    GNUGeneralPublicLicensev3_0only,
    #[strum(serialize = "gpl-3.0-or-later")]
    GNUGeneralPublicLicensev3_0orlater,
    #[strum(serialize = "gsoap-1.3b")]
    GSOAPPublicLicensev1_3b,
    #[strum(serialize = "haskellreport")]
    HaskellLanguageReportLicense,
    #[strum(serialize = "hippocratic-2.1")]
    HippocraticLicense2_1,
    #[strum(serialize = "hpnd")]
    HistoricalPermissionNoticeandDisclaimer,
    #[strum(serialize = "hpnd-sell-variant")]
    HistoricalPermissionNoticeandDisclaimersellvariant,
    #[strum(serialize = "htmltidy")]
    HTMLTidyLicense,
    #[strum(serialize = "ibm-pibs")]
    IBMPowerPCInitializationandBootSoftware,
    #[strum(serialize = "icu")]
    ICULicense,
    #[strum(serialize = "ijg")]
    IndependentJPEGGroupLicense,
    #[strum(serialize = "imagemagick")]
    ImageMagickLicense,
    #[strum(serialize = "imatix")]
    IMatixStandardFunctionLibraryAgreement,
    #[strum(serialize = "imlib2")]
    Imlib2License,
    #[strum(serialize = "info-zip")]
    InfoZIPLicense,
    #[strum(serialize = "intel")]
    IntelOpenSourceLicense,
    #[strum(serialize = "intel-acpi")]
    IntelACPISoftwareLicenseAgreement,
    #[strum(serialize = "interbase-1.0")]
    InterbasePublicLicensev1_0,
    #[strum(serialize = "ipa")]
    IPAFontLicense,
    #[strum(serialize = "ipl-1.0")]
    IBMPublicLicensev1_0,
    #[strum(serialize = "isc")]
    ISCLicense,
    #[strum(serialize = "jam")]
    JamLicense,
    #[strum(serialize = "jasper-2.0")]
    JasPerLicense,
    #[strum(serialize = "jpnic")]
    JapanNetworkInformationCenterLicense,
    #[strum(serialize = "json")]
    JSONLicense,
    #[strum(serialize = "lal-1.2")]
    LicenceArtLibre1_2,
    #[strum(serialize = "lal-1.3")]
    LicenceArtLibre1_3,
    #[strum(serialize = "latex2e")]
    Latex2eLicense,
    #[strum(serialize = "leptonica")]
    LeptonicaLicense,
    #[strum(serialize = "lgpl-2.0-only")]
    GNULibraryGeneralPublicLicensev2only,
    #[strum(serialize = "lgpl-2.0-or-later")]
    GNULibraryGeneralPublicLicensev2orlater,
    #[strum(serialize = "lgpl-2.1-only")]
    GNULesserGeneralPublicLicensev2_1only,
    #[strum(serialize = "lgpl-2.1-or-later")]
    GNULesserGeneralPublicLicensev2_1orlater,
    #[strum(serialize = "lgpl-3.0-only")]
    GNULesserGeneralPublicLicensev3_0only,
    #[strum(serialize = "lgpl-3.0-or-later")]
    GNULesserGeneralPublicLicensev3_0orlater,
    #[strum(serialize = "lgpllr")]
    LesserGeneralPublicLicenseForLinguisticResources,
    #[strum(serialize = "libpng")]
    LibpngLicense,
    #[strum(serialize = "libpng-2.0")]
    PNGReferenceLibraryversion2,
    #[strum(serialize = "libselinux-1.0")]
    LibselinuxPublicDomaiNnotice,
    #[strum(serialize = "libtiff")]
    LibtiffLicense,
    #[strum(serialize = "liliq-p-1.1")]
    LicenceLibreduQuebecPermissiveversion1_1,
    #[strum(serialize = "liliq-r-1.1")]
    LicenceLibreduQuebecReciprociteversion1_1,
    #[strum(serialize = "liliq-rplus-1.1")]
    LicenceLibreduQuebecReciprociteforteversion1_1,
    #[strum(serialize = "linux-man-pages-copyleft")]
    LinuxmanpagesCopyleft,
    #[strum(serialize = "linux-openib")]
    LinuxKernelVariantofOpenIBOrgLicense,
    #[strum(serialize = "lpl-1.0")]
    LucentPublicLicenseVersion1_0,
    #[strum(serialize = "lpl-1.02")]
    LucentPublicLicensev1_02,
    #[strum(serialize = "lppl-1.0")]
    LaTeXProjectPublicLicensev1_0,
    #[strum(serialize = "lppl-1.1")]
    LaTeXProjectPublicLicensev1_1,
    #[strum(serialize = "lppl-1.2")]
    LaTeXProjectPublicLicensev1_2,
    #[strum(serialize = "lppl-1.3a")]
    LaTeXProjectPublicLicensev1_3a,
    #[strum(serialize = "lppl-1.3c")]
    LaTeXProjectPublicLicensev1_3c,
    #[strum(serialize = "lzma-sdk-9.11-to-9.20")]
    LZMASDKLicenseVersions9_11To9_20,
    #[strum(serialize = "lzma-sdk-9.22")]
    LZMASDKLicenseVersions9_22AndBeyond,
    #[strum(serialize = "makeindex")]
    MakeIndexLicense,
    #[strum(serialize = "minpack")]
    MinpackLicense,
    #[strum(serialize = "miros")]
    TheMirOSLicence,
    #[strum(serialize = "mit")]
    MITLicense,
    #[strum(serialize = "mit-0")]
    MITNoAttribution,
    #[strum(serialize = "mit-advertising")]
    EnlightenmentLicenseE16,
    #[strum(serialize = "mit-cmu")]
    CMULicense,
    #[strum(serialize = "mit-enna")]
    EnnaLicense,
    #[strum(serialize = "mit-feh")]
    FehLicense,
    #[strum(serialize = "mit-modern-variant")]
    MITLicenseModernVariant,
    #[strum(serialize = "mit-open-group")]
    MITOpenGroupvariant,
    #[strum(serialize = "mitnfa")]
    MITNoFalseAttribsLicense,
    #[strum(serialize = "motosoto")]
    MotosotoLicense,
    #[strum(serialize = "mpi-permissive")]
    MpiPermissiveLicense,
    #[strum(serialize = "mpich2")]
    Mpich2License,
    #[strum(serialize = "mpl-1.0")]
    MozillaPublicLicense1_0,
    #[strum(serialize = "mpl-1.1")]
    MozillaPublicLicense1_1,
    #[strum(serialize = "mpl-2.0")]
    MozillaPublicLicense2_0,
    #[strum(serialize = "mpl-2.0-no-copyleft-exception")]
    MozillaPublicLicense2_0nocopyleftexception,
    #[strum(serialize = "mplus")]
    MplusFontLicense,
    #[strum(serialize = "ms-lpl")]
    MicrosoftLimitedPublicLicense,
    #[strum(serialize = "ms-pl")]
    MicrosoftPublicLicense,
    #[strum(serialize = "ms-rl")]
    MicrosoftReciprocalLicense,
    #[strum(serialize = "mtll")]
    MatrixTemplateLibraryLicense,
    #[strum(serialize = "mulanpsl-1.0")]
    MulanPermissiveSoftwareLicenseVersion1,
    #[strum(serialize = "mulanpsl-2.0")]
    MulanPermissiveSoftwareLicenseVersion2,
    #[strum(serialize = "multics")]
    MulticsLicense,
    #[strum(serialize = "mup")]
    MupLicense,
    #[strum(serialize = "naist-2003")]
    NaraInstituteofScienceandTechnologyLicense2003,
    #[strum(serialize = "nasa-1.3")]
    NASAOpenSourceAgreement1_3,
    #[strum(serialize = "naumen")]
    NaumenPublicLicense,
    #[strum(serialize = "nbpl-1.0")]
    NetBooleanPublicLicensev1,
    #[strum(serialize = "ncgl-uk-2.0")]
    NonCommercialGovernmentLicence,
    #[strum(serialize = "ncsa")]
    UniversityofIllinoisNCSAOpenSourceLicense,
    #[strum(serialize = "net-snmp")]
    NetSNMPLicense,
    #[strum(serialize = "netcdf")]
    NetCDFlicense,
    #[strum(serialize = "newsletr")]
    NewsletrLicense,
    #[strum(serialize = "ngpl")]
    NethackGeneralPublicLicense,
    #[strum(serialize = "nicta-1.0")]
    NICTAPublicSoftwareLicense,
    Version1_0,
    #[strum(serialize = "nist-pd")]
    NISTPublicDomainNotice,
    #[strum(serialize = "nist-pd-fallback")]
    NISTPublicDomainNoticewithlicensefallback,
    #[strum(serialize = "nlod-1.0")]
    NorwegianLicenceforOpenGovernmentDataNLOD1_0,
    #[strum(serialize = "nlod-2.0")]
    NorwegianLicenceforOpenGovernmentDataNLOD2_0,
    #[strum(serialize = "nlpl")]
    NoLimitPublicLicense,
    #[strum(serialize = "nokia")]
    NokiaOpenSourceLicense,
    #[strum(serialize = "nosl")]
    NetizenOpenSourceLicense,
    #[strum(serialize = "noweb")]
    NowebLicense,
    #[strum(serialize = "npl-1.0")]
    NetscapePublicLicensev1_0,
    #[strum(serialize = "npl-1.1")]
    NetscapePublicLicensev1_1,
    #[strum(serialize = "nposl-3.0")]
    NonProfitOpenSoftwareLicense3_0,
    #[strum(serialize = "nrl")]
    NRLLicense,
    #[strum(serialize = "ntp")]
    NTPLicense,
    #[strum(serialize = "ntp-0")]
    NTPNoAttribution,
    #[strum(serialize = "o-uda-1.0")]
    OpenUseofDataAgreementv1_0,
    #[strum(serialize = "occt-pl")]
    OpenCASCADETechnologyPublicLicense,
    #[strum(serialize = "oclc-2.0")]
    OCLCResearchPublicLicense2_0,
    #[strum(serialize = "odbl-1.0")]
    OpenDataCommonsOpenDatabaseLicensev1_0,
    #[strum(serialize = "odc-by-1.0")]
    OpenDataCommonsAttributionLicensev1_0,
    #[strum(serialize = "ofl-1.0")]
    SILOpenFontLicense1_0,
    #[strum(serialize = "ofl-1.0-no-rfn")]
    SILOpenFontLicense1_0withnoReservedFontName,
    #[strum(serialize = "ofl-1.0-rfn")]
    SILOpenFontLicense1_0withReservedFontName,
    #[strum(serialize = "ofl-1.1")]
    SILOpenFontLicense1_1,
    #[strum(serialize = "ofl-1.1-no-rfn")]
    SILOpenFontLicense1_1withnoReservedFontName,
    #[strum(serialize = "ofl-1.1-rfn")]
    SILOpenFontLicense1_1withReservedFontName,
    #[strum(serialize = "ogc-1.0")]
    OGCSoftwareLicenseVersion1_0,
    #[strum(serialize = "ogdl-taiwan-1.0")]
    TaiwanOpenGovernmentDataLicenseVersion1_0,
    #[strum(serialize = "ogl-canada-2.0")]
    OpenGovernmentLicenceCanada,
    #[strum(serialize = "ogl-uk-1.0")]
    OpenGovernmentLicencev1_0,
    #[strum(serialize = "ogl-uk-2.0")]
    OpenGovernmentLicencev2_0,
    #[strum(serialize = "ogl-uk-3.0")]
    OpenGovernmentLicencev3_0,
    #[strum(serialize = "ogtsl")]
    OpenGroupTestSuiteLicense,
    #[strum(serialize = "oldap-1.1")]
    OpenLDAPPublicLicensev1_1,
    #[strum(serialize = "oldap-1.2")]
    OpenLDAPPublicLicensev1_2,
    #[strum(serialize = "oldap-1.3")]
    OpenLDAPPublicLicensev1_3,
    #[strum(serialize = "oldap-1.4")]
    OpenLDAPPublicLicensev1_4,
    #[strum(serialize = "oldap-2.0")]
    OpenLDAPPublicLicensev2_0Orpossibly2_0Aand2_0B,
    #[strum(serialize = "oldap-2.0.1")]
    OpenLDAPPublicLicensev2_0_1,
    #[strum(serialize = "oldap-2.1")]
    OpenLDAPPublicLicensev2_1,
    #[strum(serialize = "oldap-2.2")]
    OpenLDAPPublicLicensev2_2,
    #[strum(serialize = "oldap-2.2.1")]
    OpenLDAPPublicLicensev2_2_1,
    #[strum(serialize = "oldap-2.2.2")]
    OpenLDAPPublicLicense2_2_2,
    #[strum(serialize = "oldap-2.3")]
    OpenLDAPPublicLicensev2_3,
    #[strum(serialize = "oldap-2.4")]
    OpenLDAPPublicLicensev2_4,
    #[strum(serialize = "oldap-2.5")]
    OpenLDAPPublicLicensev2_5,
    #[strum(serialize = "oldap-2.6")]
    OpenLDAPPublicLicensev2_6,
    #[strum(serialize = "oldap-2.7")]
    OpenLDAPPublicLicensev2_7,
    #[strum(serialize = "oldap-2.8")]
    OpenLDAPPublicLicensev2_8,
    #[strum(serialize = "oml")]
    OpenMarketLicense,
    #[strum(serialize = "openssl")]
    OpenSSLLicense,
    #[strum(serialize = "opl-1.0")]
    OpenPublicLicensev1_0,
    #[strum(serialize = "opubl-1.0")]
    OpenPublicationLicensev1_0,
    #[strum(serialize = "oset-pl-2.1")]
    OSETPublicLicenseversion2_1,
    #[strum(serialize = "osl-1.0")]
    OpenSoftwareLicense1_0,
    #[strum(serialize = "osl-1.1")]
    OpenSoftwareLicense1_1,
    #[strum(serialize = "osl-2.0")]
    OpenSoftwareLicense2_0,
    #[strum(serialize = "osl-2.1")]
    OpenSoftwareLicense2_1,
    #[strum(serialize = "osl-3.0")]
    OpenSoftwareLicense3_0,
    #[strum(serialize = "parity-6.0.0")]
    TheParityPublicLicense6_0_0,
    #[strum(serialize = "parity-7.0.0")]
    TheParityPublicLicense7_0_0,
    #[strum(serialize = "pddl-1.0")]
    OpenDataCommonsPublicDomainDedicationLicense1_0,
    #[strum(serialize = "php-3.0")]
    PHPLicensev3_0,
    #[strum(serialize = "php-3.01")]
    PHPLicensev3_01,
    #[strum(serialize = "plexus")]
    PlexusClassworldsLicense,
    #[strum(serialize = "polyform-noncommercial-1.0.0")]
    PolyFormNoncommercialLicense1_0_0,
    #[strum(serialize = "polyform-small-business-1.0.0")]
    PolyFormSmallBusinessLicense1_0_0,
    #[strum(serialize = "postgresql")]
    PostgreSQLLicense,
    #[strum(serialize = "psf-2.0")]
    PythonSoftwareFoundationLicense2_0,
    #[strum(serialize = "psfrag")]
    PsfragLicense,
    #[strum(serialize = "psutils")]
    PsutilsLicense,
    #[strum(serialize = "python-2.0")]
    PythonLicense2_0,
    #[strum(serialize = "python-2.0.1")]
    PythonLicense2_0_1,
    #[strum(serialize = "qhull")]
    QhullLicense,
    #[strum(serialize = "qpl-1.0")]
    QPublicLicense1_0,
    #[strum(serialize = "rdisc")]
    RdiscLicense,
    #[strum(serialize = "rhecos-1.1")]
    RedHateCosPublicLicensev1_1,
    #[strum(serialize = "rpl-1.1")]
    ReciprocalPublicLicense1_1,
    #[strum(serialize = "rpl-1.5")]
    ReciprocalPublicLicense1_5,
    #[strum(serialize = "rpsl-1.0")]
    RealNetworksPublicSourceLicensev1_0,
    #[strum(serialize = "rsa-md")]
    RSAMessageDigestLicense,
    #[strum(serialize = "rscpl")]
    RicohSourceCodePublicLicense,
    #[strum(serialize = "ruby")]
    RubyLicense,
    #[strum(serialize = "sax-pd")]
    SaxPublicDomainNotice,
    #[strum(serialize = "saxpath")]
    SaxpathLicense,
    #[strum(serialize = "scea")]
    SCEASharedSourceLicense,
    #[strum(serialize = "schemereport")]
    SchemeLanguageReportLicense,
    #[strum(serialize = "sendmail")]
    SendmailLicense,
    #[strum(serialize = "sendmail-8.23")]
    SendmailLicense8_23,
    #[strum(serialize = "sgi-b-1.0")]
    SGIFreeSoftwareLicenseBv1_0,
    #[strum(serialize = "sgi-b-1.1")]
    SGIFreeSoftwareLicenseBv1_1,
    #[strum(serialize = "sgi-b-2.0")]
    SGIFreeSoftwareLicenseBv2_0,
    #[strum(serialize = "shl-0.5")]
    SolderpadHardwareLicensev0_5,
    #[strum(serialize = "shl-0.51")]
    SolderpadHardwareLicense,
    Version0_51,
    #[strum(serialize = "simpl-2.0")]
    SimplePublicLicense2_0,
    #[strum(serialize = "sissl")]
    SunIndustryStandardsSourceLicensev1_1,
    #[strum(serialize = "sissl-1.2")]
    SunIndustryStandardsSourceLicensev1_2,
    #[strum(serialize = "sleepycat")]
    SleepycatLicense,
    #[strum(serialize = "smlnj")]
    StandardMLofNewJerseyLicense,
    #[strum(serialize = "smppl")]
    SecureMessagingProtocolPublicLicense,
    #[strum(serialize = "snia")]
    SNIAPublicLicense1_1,
    #[strum(serialize = "spencer-86")]
    SpencerLicense86,
    #[strum(serialize = "spencer-94")]
    SpencerLicense94,
    #[strum(serialize = "spencer-99")]
    SpencerLicense99,
    #[strum(serialize = "spl-1.0")]
    SunPublicLicensev1_0,
    #[strum(serialize = "ssh-openssh")]
    SSHOpenSSHlicense,
    #[strum(serialize = "ssh-short")]
    SSHshortnotice,
    #[strum(serialize = "sspl-1.0")]
    ServerSidePublicLicensev1,
    #[strum(serialize = "sugarcrm-1.1.3")]
    SugarCRMPublicLicensev1_1_3,
    #[strum(serialize = "swl")]
    SchemeWidgetLibrarySWLSoftwareLicenseAgreement,
    #[strum(serialize = "tapr-ohl-1.0")]
    TAPROpenHardwareLicensev1_0,
    #[strum(serialize = "tcl")]
    TCLTKLicense,
    #[strum(serialize = "tcp-wrappers")]
    TCPWrappersLicense,
    #[strum(serialize = "tmate")]
    TMateOpenSourceLicense,
    #[strum(serialize = "torque-1.1")]
    TORQUEv2_5SoftwareLicensev1_1,
    #[strum(serialize = "tosl")]
    TrussterOpenSourceLicense,
    #[strum(serialize = "tu-berlin-1.0")]
    TechnischeUniversitaetBerlinLicense1_0,
    #[strum(serialize = "tu-berlin-2.0")]
    TechnischeUniversitaetBerlinLicense2_0,
    #[strum(serialize = "ucl-1.0")]
    UpstreamCompatibilityLicensev1_0,
    #[strum(serialize = "unicode-dfs-2015")]
    UnicodeLicenseAgreementDataFilesandSoftware2015,
    #[strum(serialize = "unicode-dfs-2016")]
    UnicodeLicenseAgreementDataFilesandSoftware2016,
    #[strum(serialize = "unicode-tou")]
    UnicodeTermsofUse,
    #[strum(serialize = "unlicense")]
    TheUnlicense,
    #[strum(serialize = "upl-1.0")]
    UniversalPermissiveLicensev1_0,
    #[strum(serialize = "vim")]
    VimLicense,
    #[strum(serialize = "vostrom")]
    VOSTROMPublicLicenseforOpenSource,
    #[strum(serialize = "vsl-1.0")]
    VovidaSoftwareLicensev1_0,
    #[strum(serialize = "w3c")]
    W3CSoftwareNoticeandLicense20021231,
    #[strum(serialize = "w3c-19980720")]
    W3CSoftwareNoticeandLicense19980720,
    #[strum(serialize = "w3c-20150513")]
    W3CSoftwareNoticeandDocumentLicense20150513,
    #[strum(serialize = "watcom-1.0")]
    SybaseOpenWatcomPublicLicense1_0,
    #[strum(serialize = "wsuipa")]
    WsuipaLicense,
    #[strum(serialize = "wtfpl")]
    DoWhatTheFckYouWantToPublicLicense,
    #[strum(serialize = "x11")]
    X11License,
    #[strum(serialize = "x11-distribute-modifications-variant")]
    X11LicenseDistributionModificationVariant,
    #[strum(serialize = "xerox")]
    XeroxLicense,
    #[strum(serialize = "xfree86-1.1")]
    XFree86License1_1,
    #[strum(serialize = "xinetd")]
    XinetdLicense,
    #[strum(serialize = "xnet")]
    XNetLicense,
    #[strum(serialize = "xpp")]
    XPPLicense,
    #[strum(serialize = "xskat")]
    XSkatLicense,
    #[strum(serialize = "ypl-1.0")]
    YahooPublicLicensev1_0,
    #[strum(serialize = "ypl-1.1")]
    YahooPublicLicensev1_1,
    #[strum(serialize = "zed")]
    ZedLicense,
    #[strum(serialize = "zend-2.0")]
    ZendLicensev2_0,
    #[strum(serialize = "zimbra-1.3")]
    ZimbraPublicLicensev1_3,
    #[strum(serialize = "zimbra-1.4")]
    ZimbraPublicLicensev1_4,
    #[strum(serialize = "zlib")]
    ZlibLicense,
    #[strum(serialize = "zlib-acknowledgement")]
    ZlibLibpngLicensewithAcknowledgement,
    #[strum(serialize = "zpl-1.1")]
    ZopePublicLicense1_1,
    #[strum(serialize = "zpl-2.0")]
    ZopePublicLicense2_0,
    #[strum(serialize = "zpl-2.1")]
    ZopePublicLicense2_1,

    #[strum(disabled)]
    Other(String),
}

impl<'de> Deserialize<'de> for LicenseType {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let s = match String::deserialize(d) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        match LicenseType::from_str(s.as_str()) {
            Ok(x) => Ok(x),
            Err(_) => Ok(LicenseType::Other(s)),
        }
    }
}
