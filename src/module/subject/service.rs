use anyhow::Result;
use bytes::Bytes;
use reqwest::Method;

use crate::{
    common::model::BangumiClient,
    module::model::{ImageType, Paged},
};

use super::model::{
    DailyCalendarItem, Subject, SubjectBrowseSort, SubjectCategory, SubjectCharacter,
    SubjectPerson, SubjectSearch, SubjectSubject, SubjectType,
};

impl BangumiClient {
    /// 获取每日番剧日历
    ///
    /// 获取按星期分类的番剧条目列表，包含每日更新的番剧信息
    ///
    /// # 返回
    /// 成功返回每日日历条目列表，失败返回错误
    pub async fn get_calendar(&self) -> Result<Vec<DailyCalendarItem>> {
        let url = format!("{}/calendar", self.base_path);

        let request_builder = self.request_builder(Method::GET, &url);

        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    /// 搜索番剧条目
    ///
    /// 根据搜索条件查询番剧条目，支持分页获取结果
    ///
    /// # 参数
    /// - `limit`: 结果数量上限（可选，限制返回的条目数量）
    /// - `offset`: 结果偏移量（可选，用于分页，指定从第几条结果开始返回）
    /// - `payload`: 搜索条件（可选，包含关键词、排序方式、过滤条件等）
    ///
    /// # 返回
    /// 成功返回分页的番剧条目列表，失败返回错误
    pub async fn search_subjects(
        &self,
        limit: Option<u32>,
        offset: Option<u32>,
        payload: Option<SubjectSearch>,
    ) -> Result<Paged<Subject>> {
        let url = format!("{}/v0/search/subjects", self.base_path);
        let mut request_builder = self.request_builder(Method::POST, &url);

        if let Some(ref limit) = limit {
            request_builder = request_builder.query(&[("limit", limit)]);
        }

        if let Some(ref offset) = offset {
            request_builder = request_builder.query(&[("offset", offset)]);
        }

        let request_builder = request_builder.json(&payload);

        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    /// 获取番剧条目列表
    ///
    /// 根据筛选条件获取番剧条目列表，支持按类型、分类、时间等条件筛选
    ///
    /// # 参数
    /// - `r#type`: 条目类型（必需，指定要获取的条目类型，如动画、书籍等）
    /// - `cat`: 子类别（可选，进一步筛选条目子类别，如动画中的TV、电影等）
    /// - `series`: 是否为系列作品（可选，筛选系列作品或独立作品）
    /// - `platform`: 平台（可选，筛选特定平台的条目）
    /// - `sort`: 排序方式（可选，指定结果排序规则，如按日期、排名排序）
    /// - `year`: 年份（可选，筛选特定年份的条目）
    /// - `month`: 月份（可选，筛选特定月份的条目）
    /// - `limit`: 结果数量上限（可选，限制返回的条目数量）
    /// - `offset`: 结果偏移量（可选，用于分页，指定从第几条结果开始返回）
    ///
    /// # 返回
    /// 成功返回分页的番剧条目列表，失败返回错误
    pub async fn get_subjects(
        &self,
        r#type: SubjectType,
        cat: Option<SubjectCategory>,
        series: Option<bool>,
        platform: Option<&str>,
        sort: Option<SubjectBrowseSort>,
        year: Option<u32>,
        month: Option<u32>,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Paged<Subject>> {
        let url = format!("{}/v0/subjects", self.base_path);
        let mut req_builder = self.request_builder(reqwest::Method::GET, &url);

        req_builder = req_builder.query(&[("type", &r#type)]);
        if let Some(ref param_value) = cat {
            req_builder = req_builder.query(&[("cat", &param_value)]);
        }
        if let Some(ref param_value) = series {
            req_builder = req_builder.query(&[("series", &param_value)]);
        }
        if let Some(ref param_value) = platform {
            req_builder = req_builder.query(&[("platform", &param_value)]);
        }
        if let Some(ref param_value) = sort {
            req_builder = req_builder.query(&[("sort", &param_value)]);
        }
        if let Some(ref param_value) = year {
            req_builder = req_builder.query(&[("year", &param_value)]);
        }
        if let Some(ref param_value) = month {
            req_builder = req_builder.query(&[("month", &param_value)]);
        }
        if let Some(ref param_value) = limit {
            req_builder = req_builder.query(&[("limit", &param_value)]);
        }
        if let Some(ref param_value) = offset {
            req_builder = req_builder.query(&[("offset", &param_value)]);
        }

        let res = self.request_send(req_builder).await?.json().await?;
        Ok(res)
    }

    /// 获取单个番剧条目详情
    ///
    /// 根据条目ID获取指定番剧条目的详细信息，包括名称、简介、评分等
    ///
    /// # 参数
    /// - `subject_id`: 条目ID（必需，指定要获取详情的番剧条目）
    ///
    /// # 返回
    /// 成功返回番剧条目详情，失败返回错误
    pub async fn get_subject(&self, subject_id: u32) -> Result<Subject> {
        let url = format!("{}/v0/subjects/{subject_id}", self.base_path);

        let request_builder = self.request_builder(Method::GET, &url);

        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    /// 获取番剧条目图片
    ///
    /// 根据条目ID和图片类型获取指定番剧条目的图片数据
    ///
    /// # 参数
    /// - `subject_id`: 条目ID（必需，指定要获取图片的番剧条目）
    /// - `r#type`: 图片类型（必需，指定要获取的图片类型）
    ///
    /// # 返回
    /// 成功返回图片二进制数据，失败返回错误
    pub async fn get_subject_image(&self, subject_id: u32, r#type: ImageType) -> Result<Bytes> {
        let url = format!("{}/v0/subjects/{subject_id}/image", self.base_path);

        let mut request_builder = self.request_builder(Method::GET, &url);

        request_builder = request_builder.query(&[("type", &r#type)]);

        let res = self.request_send(request_builder).await?.bytes().await?;

        Ok(res)
    }

    /// 获取番剧条目相关人物
    ///
    /// 根据条目ID获取与该番剧条目相关的人物信息，如制作人员、声优等
    ///
    /// # 参数
    /// - `subject_id`: 条目ID（必需，指定要获取相关人物的番剧条目）
    ///
    /// # 返回
    /// 成功返回相关人物列表，失败返回错误
    pub async fn get_subject_persons(&self, subject_id: u32) -> Result<Vec<SubjectPerson>> {
        let url = format!("{}/v0/subjects/{subject_id}/persons", self.base_path);

        let request_builder = self.request_builder(Method::GET, &url);

        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    /// 获取番剧条目相关角色
    ///
    /// 根据条目ID获取与该番剧条目相关的角色信息，包括角色名称、简介及配音演员等
    ///
    /// # 参数
    /// - `subject_id`: 条目ID（必需，指定要获取相关角色的番剧条目）
    ///
    /// # 返回
    /// 成功返回相关角色列表，失败返回错误
    pub async fn get_subject_characters(&self, subject_id: u32) -> Result<Vec<SubjectCharacter>> {
        let url = format!("{}/v0/subjects/{subject_id}/characters", self.base_path);

        let request_builder = self.request_builder(Method::GET, &url);

        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }

    /// 获取番剧条目相关其他条目
    ///
    /// 根据条目ID获取与该番剧条目相关的其他番剧条目，如系列作品、衍生作品等
    ///
    /// # 参数
    /// - `subject_id`: 条目ID（必需，指定要获取相关条目的番剧条目）
    ///
    /// # 返回
    /// 成功返回相关条目列表，失败返回错误
    pub async fn get_subject_subjects(&self, subject_id: u32) -> Result<Vec<SubjectSubject>> {
        let url = format!("{}/v0/subjects/{subject_id}/subjects", self.base_path);

        let request_builder = self.request_builder(Method::GET, &url);

        let res = self.request_send(request_builder).await?.json().await?;

        Ok(res)
    }
}
